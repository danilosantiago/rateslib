# This file contains bond convention outlines
from __future__ import annotations

from datetime import datetime

from rateslib import defaults
from rateslib.calendars import add_tenor, dcf
from rateslib.curves import index_left
from rateslib.default import NoInput
from rateslib.dual import DualTypes
from rateslib.instruments.bonds.accrual_conventions import (
    _acc_30e360,
    _acc_act365_with_1y_and_stub_adjustment,
    _acc_linear_proportion_by_days,
    _acc_linear_proportion_by_days_long_stub_split,
)
from rateslib.instruments.bonds.discount_conventions import (
    _v1_comp_stub_act365f,
    _v1_compounded_by_remaining_accrual_frac_except_simple_final_period,
    _v1_compounded_by_remaining_accrual_fraction,
    _v1_simple,
    _v1_simple_1y_adjustment,
    _v2_,
    _v2_annual,
    _v3_30e360_u_simple,
    _v3_compounded,
    _v3_simple,
)

ACC_FRAC_FUNCS = {
    "linear_days": _acc_linear_proportion_by_days,
    "linear_days_long_front_split": _acc_linear_proportion_by_days_long_stub_split,
    "30e360": _acc_30e360,
    "act365f_1y": _acc_act365_with_1y_and_stub_adjustment,
}

V1_FUNCS = {
    "compounding": _v1_compounded_by_remaining_accrual_fraction,
    "compounding_final_simple": _v1_compounded_by_remaining_accrual_frac_except_simple_final_period,
    "compounding_stub_act365f": _v1_comp_stub_act365f,
    "simple": _v1_simple,
    "simple_long_stub_compounding": _v1_simple_1y_adjustment,
}

V2_FUNCS = {
    "regular": _v2_,
    "annual": _v2_annual,
}

V3_FUNCS = {
    "compounding": _v3_compounded,
    "simple": _v3_simple,
    "simple_30e360": _v3_30e360_u_simple,
}


def _get_bond_calc_mode(calc_mode: str | BondCalcMode) -> BondCalcMode:
    if isinstance(calc_mode, str):
        return BOND_MODE_MAP[calc_mode.lower()]
    return calc_mode


class BondCalcMode:
    """
    Define calculation conventions for :class:`~rateslib.instruments.FixedRateBond` type.

    Parameters
    ----------
    accrual_type: str,
        The calculation type for accrued interest.
    v1_type: str
        The calculation function that defines discounting of the first period of the YTM formula.
    v2_type: str
        The calculation function that defines discounting of the regular periods of the YTM formula.
    v3_type: str
        The calculation function that defines discounting of the last period of the YTM formula.

    Notes
    -------

    **Accrual Functions**

    These functions return the **fraction** of a bond cashflow that is attributed to the settlement
    date, in order to determine accrued interest. The available input options are;

    - *"linear_days"*: Measures a calendar day, linear proportion between unadjusted start and
      end coupon dates of the coupon period, and applies that proportion to the cashflow, which is
      calculated separately using the conventions for the bond. (Typically used by many bonds, e.g.
      UK and German GBs)

      .. math::

         &\\text{Accrual fraction} = r / s \\\\
         &\\text{where,} \\\\
         &r = \\text{Calendar days between last coupon (unadjusted) and settlement} \\\\
         &s = \\text{Calendar days between unadjusted coupon dates} \\\\

    - *"linear_days_long_front_split"*: Is the same as above, **except** in the case of long
      stub periods, which are treated as front stubs. (Primarily implemented to satisfy the
      US Treasury calculations in Section 31B ii A.356)
    - *"30e360"*: Ignores the coupon convention on the bond and calculates accrued from the
      unadjusted last coupon date to settlement with a 30e360 day count convention, **except**
      stubs revert to *'linear_days'*. (Used by Swedish GBs)

      .. math::

         &\\text{Accrual fraction} =  1 - d f  \\\\
         &\\text{where,} \\\\
         &d = \\text{30e360 DCF between settlement and next unadjusted coupon date} \\\\
         &f = \\text{Number of regular coupon periods per year} \\\\

    - *"Act365_1y"*: Ignores the coupon convention on the bond and calculates accrued from
      the unadjusted last coupon date to settlement with an Act365F day count convention. Stub
      periods are adjusted to use *'linear_days'* and periods longer than 1y have additional
      adjustment. (Used by Canadian GBs)

      .. math::

         & r = s \\qquad \\implies \\quad \\text{Accrual fraction} =  1.0  \\\\
         & r > 365 / f \\qquad \\implies \\quad \\text{Accrual fraction} =  1.0 - f(s-r) / 365 \\\\
         & r \\le 365 / f \\qquad \\implies \\quad \\text{Accrual fraction} =  rf / 365 \\\\

    **Discounting Functions for YTM Calculation**

    Yield-to-maturity is calculated using the below formula, where specific functions derive
    some values based on the conventions of a given bond.

    .. math::

       P &= v_1 \\left ( c_1 + 100 \\right ), \\quad n = 1 \\\\
       P &= v_1 \\left ( c_1 + v3(c_2 + 100) \\right ), \\quad n = 2 \\\\
       P &= v_1 \\left ( \\sum_{i=1}^{n-1} c_i v_2^{i-1} + c_nv_2^{n-2}v_3 + 100 v_2^{n-2}v_3 \\right ), \\quad n > 1  \\\\
    where,

    .. math::

       P &= \\text{Dirty price}, \\; n = \\text{Coupon periods remaining} \\\\
       c_1 &= \\text{Cashflow (per 100) on next coupon date (may be zero if ex-dividend)} \\\\
       c_i &= i \\text{'th cashflow (per 100) on subsequent coupon dates} \\\\
       v_1 &= \\text{Discount value for the initial, possibly stub, period} \\\\
       v_2 &= \\text{Discount value for the interim regular periods} \\\\
       v_3 &= \\text{Discount value for the final, possibly stub, period} \\\\

    **v1** Functions

    """  # noqa: E501

    def __init__(
        self,
        settle_accrual_type: str,
        ytm_accrual_type: str,
        v1_type: str,
        v2_type: str,
        v3_type: str,
    ):
        self._settle_acc_frac_func = ACC_FRAC_FUNCS[settle_accrual_type.lower()]
        self._ytm_acc_frac_func = ACC_FRAC_FUNCS[ytm_accrual_type.lower()]
        self._v1 = V1_FUNCS[v1_type.lower()]
        self._v2 = V2_FUNCS[v2_type.lower()]
        self._v3 = V3_FUNCS[v3_type.lower()]

        self._kwargs: dict = {
            "settle_accrual": settle_accrual_type,
            "ytm_accrual": ytm_accrual_type,
            "v1": v1_type,
            "v2": v2_type,
            "v3": v3_type,
        }

    @property
    def kwargs(self) -> dict:
        """
        Return the named input parameters for the *BondCalcMode*.
        """
        return self._kwargs


class BillCalcMode:

    def __init__(
        self,
        price_type: str,
        price_accrual_type: str,
        ytm_clone_kwargs: dict | str,
    ):
        self._price_type = price_type
        self._price_accrual_frac_func = ACC_FRAC_FUNCS[price_accrual_type.lower()]
        if isinstance(ytm_clone_kwargs, dict):
            self._ytm_clone_kwargs = ytm_clone_kwargs
        else:
            self._ytm_clone_kwargs = defaults.spec[ytm_clone_kwargs]
        self._kwargs = {
            "price_type": price_type,
            "price_accrual_type": price_accrual_type,
            "ytm_clone": "Custom dict" if isinstance(ytm_clone_kwargs, dict) else ytm_clone_kwargs
        }

    @property
    def kwargs(self):
        return self._kwargs


UK_GB = BondCalcMode(
    # UK government bond conventions
    settle_accrual_type="linear_days",
    ytm_accrual_type="linear_days",
    v1_type="compounding",
    v2_type="regular",
    v3_type="compounding",
)

US_GB = BondCalcMode(
    # US Treasury street convention
    settle_accrual_type="linear_days_long_front_split",
    ytm_accrual_type="linear_days_long_front_split",
    v1_type="compounding",
    v2_type="regular",
    v3_type="compounding",
)

US_GB_TSY = BondCalcMode(
    # US Treasury treasury convention
    settle_accrual_type="linear_days_long_front_split",
    ytm_accrual_type="linear_days_long_front_split",
    v1_type="simple_long_stub_compounding",
    v2_type="regular",
    v3_type="compounding",
)

SE_GB = BondCalcMode(
    # Swedish government bonds
    settle_accrual_type="30e360",
    ytm_accrual_type="30e360",
    v1_type="compounding",
    v2_type="regular",
    v3_type="simple_30e360",
)

CA_GB = BondCalcMode(
    # Canadian government bonds
    settle_accrual_type="act365f_1y",
    ytm_accrual_type="linear_days",
    v1_type="compounding",
    v2_type="regular",
    v3_type="simple_30e360",
)

DE_GB = BondCalcMode(
    # German government bonds
    settle_accrual_type="linear_days",
    ytm_accrual_type="linear_days",
    v1_type="compounding_final_simple",
    v2_type="regular",
    v3_type="compounding",
)

FR_GB = BondCalcMode(
    # French OATs
    settle_accrual_type="linear_days",
    ytm_accrual_type="linear_days",
    v1_type="compounding",
    v2_type="regular",
    v3_type="compounding",
)

IT_GB = BondCalcMode(
    # Italian GBs
    settle_accrual_type="linear_days",
    ytm_accrual_type="linear_days",
    v1_type="compounding_final_simple",
    v2_type="annual",
    v3_type="compounding",
)

NO_GB = BondCalcMode(
    # Norwegian GBs
    settle_accrual_type="act365f_1y",
    ytm_accrual_type="act365f_1y",
    v1_type="compounding_stub_act365f",
    v2_type="regular",
    v3_type="compounding",
)

NL_GB = BondCalcMode(
    # Dutch GBs
    settle_accrual_type="linear_days_long_front_split",
    ytm_accrual_type="linear_days_long_front_split",
    v1_type="compounding_final_simple",
    v2_type="regular",
    v3_type="compounding",
)

UK_GBB = BillCalcMode(
    # UK T-bills
    price_type="simple",
    price_accrual_type="linear_days",
    ytm_clone_kwargs="uk_gb",
)

US_GBB = BillCalcMode(
    # US T-bills
    price_type="discount",
    price_accrual_type="linear_days",
    ytm_clone_kwargs="us_gb",
)

SE_GBB = BillCalcMode(
    # Swedish T-bills
    price_type="simple",
    price_accrual_type="linear_days",
    ytm_clone_kwargs="se_gb",
)

BOND_MODE_MAP = {
    "uk_gb": UK_GB,
    "us_gb": US_GB,
    "de_gb": DE_GB,
    "fr_gb": FR_GB,
    "nl_gb": NL_GB,
    "no_gb": NO_GB,
    "se_gb": SE_GB,
    "us_gb_tsy": US_GB_TSY,
    "it_gb": IT_GB,
    "ca_gb": CA_GB,

    # aliases
    "ukg": UK_GB,
    "cadgb": CA_GB,
    "ust": US_GB,
    "ust_31bii": US_GB_TSY,
    "sgb": SE_GB
}

BILL_MODE_MAP = {
    "uk_gbb": UK_GBB,
    "us_gbb": US_GBB,
    "se_gbb": SE_GBB,

    # aliases
    "ustb": US_GBB,
    "uktb": UK_GBB,
    "sgbb": SE_GBB,
}
