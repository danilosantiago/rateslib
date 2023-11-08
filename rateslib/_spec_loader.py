import pandas as pd
import os
from packaging import version

DEVELOPMENT = True
if DEVELOPMENT:
    # DEVELOPMENT mode is used to load and create instrument specs from a CSV file.
    # This is loaded by default and slower to parse than directly creating a dict
    # So when packaging output the INSTRUMENT_SPEC dict and paste into the non-development
    # section.

    def _append_kwargs_name(df):
        """combine the columns leg and kwargs to produce library consistent kwargs for dicts"""
        prefix = df["leg"]
        prefix = prefix.where(prefix == "leg2", "")
        prefix = prefix.replace("leg2", "leg2_")
        df["kwargs_name"] = prefix + df["kwarg"]
        return df.set_index("kwargs_name")

    def _parse_bool(df):
        """parse data input as bools to return True and False dtypes."""

        def _map_true_false(v):
            try:
                if v.upper() == "TRUE":
                    return True
                elif v.upper() == "FALSE":
                    return False
            except AttributeError:
                return None
            else:
                return None

        if version.parse(pd.__version__) >= version.parse("2.1.0"):
            # applymap issues a deprecation warning with version <2.1.0
            # TODO (low): clean this up when setting a minimum pandas version at 2.1.0
            df[df["dtype"] == "bool"] = df[df["dtype"] == "bool"].map(_map_true_false)
        else:
            df[df["dtype"] == "bool"] = df[df["dtype"] == "bool"].applymap(_map_true_false)
        return df

    path = "data/instrument_spec.csv"
    abspath = os.path.dirname(os.path.abspath(__file__))
    target = os.path.join(abspath, path)
    df = pd.read_csv(target)
    df = _append_kwargs_name(df)
    df = _parse_bool(df)
    df_legs = df[~(df["leg"] == "meta")]

    DTYPE_MAP = {
        "str": str,
        "float": float,
        "bool": bool,
        "int": int,
    }

    def _map_dtype(v):
        try:
            return DTYPE_MAP[v]
        except KeyError:
            return v

    def _map_str_int(v):
        try:
            return int(v)
        except ValueError:
            return v

    def _get_kwargs(spec):
        """From the legs DataFrame extract the relevant column and ensure dtypes are suitable."""
        # get values that are not null
        s = df_legs[spec]
        s = s[pd.notna(s)]
        # assign the correct dtypes for the values
        dfs = s.to_frame().transpose()
        dtypes = df.loc[s.index, "dtype"]
        dtypes = dtypes.map(_map_dtype)
        dfs = dfs.astype(dtype=dtypes.to_dict(), errors="raise")
        # rotate and return values in a dict
        s = dfs.transpose()[spec]
        d = s.to_dict()

        # roll dtype is str or int causes object issues
        if "roll" in d:
            d["roll"] = _map_str_int(d["roll"])
        if "leg2_roll" in d:
            d["leg2_roll"] = _map_str_int(d["leg2_roll"])
        return d

    INSTRUMENT_SPECS = {
        "usd_irs": _get_kwargs("usd_irs"),  # IRS
        "gbp_irs": _get_kwargs("gbp_irs"),
        "eur_irs": _get_kwargs("eur_irs"),
        "eur_irs3": _get_kwargs("eur_irs3"),
        "eur_irs6": _get_kwargs("eur_irs6"),
        "eur_irs1": _get_kwargs("eur_irs1"),
        "sek_irs": _get_kwargs("sek_irs"),
        "sek_irs3": _get_kwargs("sek_irs3"),
        "nok_irs": _get_kwargs("nok_irs"),
        "nok_irs3": _get_kwargs("nok_irs3"),
        "nok_irs6": _get_kwargs("nok_irs6"),
        "chf_irs": _get_kwargs("chf_irs"),
        "eur_fra3": _get_kwargs("eur_fra3"),  # FRA
        "eur_fra6": _get_kwargs("eur_fra6"),
        "sek_fra3": _get_kwargs("sek_fra3"),
        "eur_sbs36": _get_kwargs("eur_sbs36"),  # SBS
        "eurusd_xcs": _get_kwargs("eurusd_xcs"),  # XCS
        "gbpusd_xcs": _get_kwargs("gbpusd_xcs"),
        "eurgbp_xcs": _get_kwargs("eurgbp_xcs"),
        "eur_zcis": _get_kwargs("eur_zcis"),  # ZCIS
        "gbp_zcis": _get_kwargs("gbp_zcis"),
        "usd_zcis": _get_kwargs("usd_zcis"),
        "gbp_zcs": _get_kwargs("gbp_zcs"),  # ZCS
        "sek_iirs": _get_kwargs("sek_iirs"),  # IIRS
        "usd_gb": _get_kwargs("usd_gb"),  # FRB
        "usd_gbb": _get_kwargs("usd_gbb"),
        "gbp_gb": _get_kwargs("gbp_gb"),
        "cad_gb": _get_kwargs("cad_gb"),
        "sek_gb": _get_kwargs("sek_gb"),
        "sek_gbb": _get_kwargs("sek_gbb"),
        "usd_frn5": _get_kwargs("usd_frn5"),  # FRN
        "usd_stir": _get_kwargs("usd_stir"),  # STIR Futures
        "usd_stir1": _get_kwargs("usd_stir1"),
        "eur_stir": _get_kwargs("eur_stir"),
        "eur_stir1": _get_kwargs("eur_stir1"),
        "eur_stir3": _get_kwargs("eur_stir3"),
        "gbp_stir": _get_kwargs("gbp_stir"),
        "test": _get_kwargs("test"),  # TEST
    }

    # add some aliases

    INSTRUMENT_SPECS = {
        **INSTRUMENT_SPECS,
        "sofr": INSTRUMENT_SPECS["usd_irs"],
        "ust": INSTRUMENT_SPECS["usd_gb"],
        "ustb": INSTRUMENT_SPECS["usd_gbb"],
        "ukt": INSTRUMENT_SPECS["gbp_gb"],
        "ukti": INSTRUMENT_SPECS["gbp_gbi"],
        "gilt": INSTRUMENT_SPECS["gbp_gb"],
        "cadgb": INSTRUMENT_SPECS["cad_gb"],
        "sgb": INSTRUMENT_SPECS["sek_gb"],
        "sgbb": INSTRUMENT_SPECS["sek_gbb"],
        "sofr3mf": INSTRUMENT_SPECS["usd_stir"],
        "sofr1mf": INSTRUMENT_SPECS["usd_stir1"],
        "sonia3mf": INSTRUMENT_SPECS["gbp_stir"],
        "estr1mf": INSTRUMENT_SPECS["eur_stir1"],
        "estr3mf": INSTRUMENT_SPECS["eur_stir"],
        "euribor3mf": INSTRUMENT_SPECS["eur_stir3"],
    }

else:
    INSTRUMENT_SPECS = {
        "usd_irs": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "nyc",
            "payment_lag": 2,
            "currency": "usd",
            "convention": "act360",
            "leg2_frequency": "a",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "nyc",
            "leg2_payment_lag": 2,
            "leg2_currency": "usd",
            "leg2_convention": "act360",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "rfr_payment_delay",
        },
        "gbp_irs": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": True,
            "modifier": "mf",
            "calendar": "ldn",
            "payment_lag": 0,
            "currency": "gbp",
            "convention": "act365f",
            "leg2_frequency": "a",
            "leg2_stub": "shortfront",
            "leg2_eom": True,
            "leg2_modifier": "mf",
            "leg2_calendar": "ldn",
            "leg2_payment_lag": 0,
            "leg2_currency": "gbp",
            "leg2_convention": "act365f",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "rfr_payment_delay",
        },
        "eur_irs": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "tgt",
            "payment_lag": 1,
            "currency": "eur",
            "convention": "act360",
            "leg2_frequency": "a",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "tgt",
            "leg2_payment_lag": 1,
            "leg2_currency": "eur",
            "leg2_convention": "act360",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "rfr_payment_delay",
        },
        "eur_irs3": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "tgt",
            "payment_lag": 0,
            "currency": "eur",
            "convention": "30e360",
            "leg2_frequency": "q",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "tgt",
            "leg2_payment_lag": 0,
            "leg2_currency": "eur",
            "leg2_convention": "act360",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "ibor",
        },
        "eur_irs6": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "tgt",
            "payment_lag": 0,
            "currency": "eur",
            "convention": "30e360",
            "leg2_frequency": "s",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "tgt",
            "leg2_payment_lag": 0,
            "leg2_currency": "eur",
            "leg2_convention": "act360",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "ibor",
        },
        "sek_irs": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "stk",
            "payment_lag": 1,
            "currency": "sek",
            "convention": "act360",
            "leg2_frequency": "a",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "stk",
            "leg2_payment_lag": 1,
            "leg2_currency": "sek",
            "leg2_convention": "act360",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "rfr_payment_delay",
        },
        "sek_irs3": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "stk",
            "payment_lag": 0,
            "currency": "sek",
            "convention": "30e360",
            "leg2_frequency": "q",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "stk",
            "leg2_payment_lag": 0,
            "leg2_currency": "sek",
            "leg2_convention": "act360",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "ibor",
        },
        "nok_irs": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "osl",
            "payment_lag": 2,
            "currency": "nok",
            "convention": "act365f",
            "leg2_frequency": "a",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "osl",
            "leg2_payment_lag": 2,
            "leg2_currency": "nok",
            "leg2_convention": "act365f",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "rfr_payment_delay",
        },
        "nok_irs3": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "stk",
            "payment_lag": 0,
            "currency": "sek",
            "convention": "30e360",
            "leg2_frequency": "q",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "stk",
            "leg2_payment_lag": 0,
            "leg2_currency": "nok",
            "leg2_convention": "act360",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "ibor",
        },
        "nok_irs6": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "osl",
            "payment_lag": 0,
            "currency": "nok",
            "convention": "30e360",
            "leg2_frequency": "s",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "osl",
            "leg2_payment_lag": 0,
            "leg2_currency": "nok",
            "leg2_convention": "act360",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "ibor",
        },
        "chf_irs": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "zur",
            "payment_lag": 2,
            "currency": "chf",
            "convention": "act360",
            "leg2_frequency": "a",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "zur",
            "leg2_payment_lag": 2,
            "leg2_currency": "chf",
            "leg2_convention": "act360",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "rfr_payment_delay",
        },
        "eur_fra3": {
            "frequency": "q",
            "eom": False,
            "modifier": "mf",
            "calendar": "tgt",
            "payment_lag": 0,
            "currency": "eur",
            "convention": "act360",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "ibor",
            "leg2_method_param": 2,
        },
        "eur_fra6": {
            "frequency": "s",
            "eom": False,
            "modifier": "mf",
            "calendar": "tgt",
            "payment_lag": 0,
            "currency": "eur",
            "convention": "act360",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "ibor",
            "leg2_method_param": 2,
        },
        "sek_fra3": {
            "frequency": "q",
            "eom": False,
            "modifier": "mf",
            "calendar": "stk",
            "payment_lag": 0,
            "currency": "sek",
            "convention": "act360",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "ibor",
            "leg2_method_param": 2,
        },
        "eur_sbs36": {
            "frequency": "q",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "tgt",
            "payment_lag": 0,
            "currency": "eur",
            "convention": "act360",
            "leg2_frequency": "s",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "tgt",
            "leg2_payment_lag": 0,
            "leg2_currency": "eur",
            "leg2_convention": "act360",
            "spread_compound_method": "none_simple",
            "fixing_method": "ibor",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "ibor",
        },
        "eurusd_xcs": {
            "frequency": "q",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "tgt,nyc",
            "payment_lag": 2,
            "currency": "eur",
            "convention": "act360",
            "leg2_frequency": "q",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "tgt,nyc",
            "leg2_payment_lag": 2,
            "leg2_currency": "usd",
            "leg2_convention": "act360",
            "spread_compound_method": "none_simple",
            "fixing_method": "rfr_payment_delay",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "rfr_payment_delay",
            "initial_exchange": True,
            "final_exchange": True,
            "leg2_initial_exchange": True,
            "leg2_final_exchange": True,
            "payment_lag_exchange": 0,
            "leg2_payment_lag_exchange": 0,
            "fixed": False,
            "leg2_fixed": False,
            "leg2_mtm": True,
        },
        "gbpusd_xcs": {
            "frequency": "q",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "ldn,nyc",
            "payment_lag": 2,
            "currency": "gbp",
            "convention": "act365f",
            "leg2_frequency": "q",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "ldn,nyc",
            "leg2_payment_lag": 2,
            "leg2_currency": "usd",
            "leg2_convention": "act360",
            "spread_compound_method": "none_simple",
            "fixing_method": "rfr_payment_delay",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "rfr_payment_delay",
            "initial_exchange": True,
            "final_exchange": True,
            "leg2_initial_exchange": True,
            "leg2_final_exchange": True,
            "payment_lag_exchange": 0,
            "leg2_payment_lag_exchange": 0,
            "fixed": False,
            "leg2_fixed": False,
            "leg2_mtm": True,
        },
        "eurgbp_xcs": {
            "frequency": "q",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "tgt,ldn",
            "payment_lag": 2,
            "currency": "eur",
            "convention": "act360",
            "leg2_frequency": "q",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "tgt,ldn",
            "leg2_payment_lag": 2,
            "leg2_currency": "gbp",
            "leg2_convention": "act365f",
            "spread_compound_method": "none_simple",
            "fixing_method": "rfr_payment_delay",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "rfr_payment_delay",
            "initial_exchange": True,
            "final_exchange": True,
            "leg2_initial_exchange": True,
            "leg2_final_exchange": True,
            "payment_lag_exchange": 0,
            "leg2_payment_lag_exchange": 0,
            "fixed": False,
            "leg2_fixed": False,
            "leg2_mtm": True,
        },
        "eur_zcis": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "tgt",
            "payment_lag": 0,
            "currency": "eur",
            "convention": "1+",
            "leg2_frequency": "a",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "tgt",
            "leg2_payment_lag": 0,
            "leg2_currency": "eur",
            "leg2_convention": "1+",
            "leg2_index_method": "monthly",
            "leg2_index_lag": 3,
        },
        "gbp_zcis": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "ldn",
            "payment_lag": 0,
            "currency": "gbp",
            "convention": "1+",
            "leg2_frequency": "a",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "ldn",
            "leg2_payment_lag": 0,
            "leg2_currency": "gbp",
            "leg2_convention": "1+",
            "leg2_index_method": "monthly",
            "leg2_index_lag": 2,
        },
        "usd_zcis": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "nyc",
            "payment_lag": 0,
            "currency": "usd",
            "convention": "1+",
            "leg2_frequency": "a",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "nyc",
            "leg2_payment_lag": 0,
            "leg2_currency": "usd",
            "leg2_convention": "1+",
            "leg2_index_method": "daily",
            "leg2_index_lag": 3,
        },
        "gbp_zcs": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": True,
            "modifier": "mf",
            "calendar": "ldn",
            "payment_lag": 0,
            "currency": "gbp",
            "convention": "act365f",
            "leg2_frequency": "a",
            "leg2_stub": "shortfront",
            "leg2_eom": True,
            "leg2_modifier": "mf",
            "leg2_calendar": "ldn",
            "leg2_payment_lag": 0,
            "leg2_currency": "gbp",
            "leg2_convention": "act365f",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "rfr_payment_delay",
        },
        "sek_iirs": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "none",
            "calendar": "stk",
            "payment_lag": 0,
            "currency": "sek",
            "convention": "actacticma",
            "leg2_frequency": "q",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "stk",
            "leg2_payment_lag": 0,
            "leg2_currency": "sek",
            "leg2_convention": "act360",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "ibor",
            "index_method": "daily",
            "index_lag": 3,
        },
        "usd_gb": {
            "frequency": "s",
            "stub": "shortfront",
            "eom": False,
            "modifier": "none",
            "calendar": "nyc",
            "payment_lag": 0,
            "currency": "usd",
            "convention": "actacticma",
            "initial_exchange": False,
            "final_exchange": True,
            "payment_lag_exchange": 0,
            "settle": 1,
            "ex_div": 1,
            "calc_mode": "ust",
        },
        "usd_gbb": {
            "modifier": "none",
            "calendar": "nyc",
            "payment_lag": 0,
            "currency": "usd",
            "convention": "act360",
            "initial_exchange": False,
            "final_exchange": True,
            "payment_lag_exchange": 0,
            "settle": 1,
            "ex_div": 0,
            "calc_mode": "ustb",
        },
        "gbp_gb": {
            "frequency": "s",
            "stub": "shortfront",
            "eom": False,
            "modifier": "none",
            "calendar": "ldn",
            "payment_lag": 0,
            "currency": "gbp",
            "convention": "actacticma",
            "initial_exchange": False,
            "final_exchange": True,
            "payment_lag_exchange": 0,
            "settle": 1,
            "ex_div": 7,
            "calc_mode": "ukg",
        },
        "cad_gb": {
            "frequency": "s",
            "stub": "shortfront",
            "eom": False,
            "modifier": "none",
            "calendar": "tro",
            "payment_lag": 0,
            "currency": "cad",
            "convention": "actacticma_stub365f",
            "initial_exchange": False,
            "final_exchange": True,
            "payment_lag_exchange": 0,
            "settle": 1,
            "ex_div": 1,
            "calc_mode": "cadgb",
        },
        "sek_gb": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "none",
            "calendar": "stk",
            "payment_lag": 0,
            "currency": "sek",
            "convention": "actacticma",
            "initial_exchange": False,
            "final_exchange": True,
            "payment_lag_exchange": 0,
            "settle": 1,
            "ex_div": 5,
            "calc_mode": "sgb",
        },
        "sek_gbb": {
            "modifier": "none",
            "calendar": "stk",
            "payment_lag": 0,
            "currency": "sek",
            "convention": "act360",
            "initial_exchange": False,
            "final_exchange": True,
            "payment_lag_exchange": 0,
            "settle": 1,
            "ex_div": 0,
            "calc_mode": "sgbb",
        },
        "usd_frn5": {
            "frequency": "q",
            "eom": False,
            "modifier": "mf",
            "calendar": "nyc",
            "payment_lag": 0,
            "currency": "usd",
            "convention": "act360",
            "spread_compound_method": "none_simple",
            "fixing_method": "rfr_observation_shift",
            "method_param": 5,
            "settle": 1,
            "ex_div": 1,
        },
        "test": {
            "frequency": "m",
            "stub": "longfront",
            "eom": False,
            "modifier": "p",
            "calendar": "nyc,tgt,ldn",
            "payment_lag": 4,
            "currency": "tes",
            "convention": "test",
            "leg2_frequency": "m",
            "leg2_stub": "longback",
            "leg2_roll": 1,
            "leg2_eom": False,
            "leg2_modifier": "mp",
            "leg2_calendar": "nyc,tgt,ldn",
            "leg2_payment_lag": 3,
            "leg2_convention": "test2",
        },
        "sofr": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "mf",
            "calendar": "nyc",
            "payment_lag": 2,
            "currency": "usd",
            "convention": "act360",
            "leg2_frequency": "a",
            "leg2_stub": "shortfront",
            "leg2_eom": False,
            "leg2_modifier": "mf",
            "leg2_calendar": "nyc",
            "leg2_payment_lag": 2,
            "leg2_currency": "usd",
            "leg2_convention": "act360",
            "leg2_spread_compound_method": "none_simple",
            "leg2_fixing_method": "rfr_payment_delay",
        },
        "ust": {
            "frequency": "s",
            "stub": "shortfront",
            "eom": False,
            "modifier": "none",
            "calendar": "nyc",
            "payment_lag": 0,
            "currency": "usd",
            "convention": "actacticma",
            "initial_exchange": False,
            "final_exchange": True,
            "payment_lag_exchange": 0,
            "settle": 1,
            "ex_div": 1,
            "calc_mode": "ust",
        },
        "ustb": {
            "modifier": "none",
            "calendar": "nyc",
            "payment_lag": 0,
            "currency": "usd",
            "convention": "act360",
            "initial_exchange": False,
            "final_exchange": True,
            "payment_lag_exchange": 0,
            "settle": 1,
            "ex_div": 0,
            "calc_mode": "ustb",
        },
        "ukt": {
            "frequency": "s",
            "stub": "shortfront",
            "eom": False,
            "modifier": "none",
            "calendar": "ldn",
            "payment_lag": 0,
            "currency": "gbp",
            "convention": "actacticma",
            "initial_exchange": False,
            "final_exchange": True,
            "payment_lag_exchange": 0,
            "settle": 1,
            "ex_div": 7,
            "calc_mode": "ukg",
        },
        "gilt": {
            "frequency": "s",
            "stub": "shortfront",
            "eom": False,
            "modifier": "none",
            "calendar": "ldn",
            "payment_lag": 0,
            "currency": "gbp",
            "convention": "actacticma",
            "initial_exchange": False,
            "final_exchange": True,
            "payment_lag_exchange": 0,
            "settle": 1,
            "ex_div": 7,
            "calc_mode": "ukg",
        },
        "cadgb": {
            "frequency": "s",
            "stub": "shortfront",
            "eom": False,
            "modifier": "none",
            "calendar": "tro",
            "payment_lag": 0,
            "currency": "cad",
            "convention": "actacticma_stub365f",
            "initial_exchange": False,
            "final_exchange": True,
            "payment_lag_exchange": 0,
            "settle": 1,
            "ex_div": 1,
            "calc_mode": "cadgb",
        },
        "sgb": {
            "frequency": "a",
            "stub": "shortfront",
            "eom": False,
            "modifier": "none",
            "calendar": "stk",
            "payment_lag": 0,
            "currency": "sek",
            "convention": "actacticma",
            "initial_exchange": False,
            "final_exchange": True,
            "payment_lag_exchange": 0,
            "settle": 1,
            "ex_div": 5,
            "calc_mode": "sgb",
        },
        "sgbb": {
            "modifier": "none",
            "calendar": "stk",
            "payment_lag": 0,
            "currency": "sek",
            "convention": "act360",
            "initial_exchange": False,
            "final_exchange": True,
            "payment_lag_exchange": 0,
            "settle": 1,
            "ex_div": 0,
            "calc_mode": "sgbb",
        },
    }
