import math

import numpy as np

from rateslib import defaults
from rateslib.default import NoInput
from rateslib.rs import Dual, Dual2

PRECISION = 1e-14
FLOATS = (float, np.float16, np.float32, np.float64, np.longdouble)
INTS = (int, np.int8, np.int16, np.int32, np.int32, np.int64)


class Variable:
    """
    A user defined, exogenous variable that automatically converts to a
    :class:`~rateslib.variable.Dual` or
    :class:`~rateslib.variable.Dual2` type dependent upon the overall AD calculation order.

    Parameters
    ----------
    real : float
        The real coefficient of the dual number
    vars : tuple of str, optional
        The labels of the variables for which to record derivatives. If not given
        the dual number represents a constant, equivalent to an int or float.
    dual : 1d ndarray, optional
        First derivative information contained as coefficient of linear manifold.
        Defaults to an array of ones the length of ``vars`` if not given.

    Attributes
    ----------
    real : float
    vars : str, tuple of str
    dual : 1d ndarray
    """

    def __init__(
        self,
        real: float,
        vars: tuple[str, ...] = (),
        dual: np.ndarray | NoInput = NoInput(0),
    ):
        self.real: float = float(real)
        self.vars: tuple[str, ...] = tuple(vars)
        n = len(self.vars)
        if dual is NoInput.blank or len(dual) == 0:
            self.dual: np.ndarray = np.ones(n)
        else:
            self.dual = np.asarray(dual.copy())

    def _to_dual_type(self, order):
        if order == 1:
            return Dual(self.real, vars=self.vars, dual=self.dual)
        elif order == 2:
            return Dual2(self.real, vars=self.vars, dual=self.dual, dual2=[])
        else:
            raise TypeError(
                f"`Variable` can only be converted with `order` in [1, 2], got order: {order}."
            )

    def __eq__(self, argument):
        """
        Compare an argument with a Variable for equality.
        This does not account for variable ordering.
        """
        if not isinstance(argument, type(self)):
            return False
        if self.vars == argument.vars:
            return self.__eq_coeffs__(argument, PRECISION)
        return False

    def __eq_coeffs__(self, argument, precision):
        """Compare the coefficients of two dual array numbers for equality."""
        return not (
            not math.isclose(self.real, argument.real, abs_tol=precision)
            or not np.all(np.isclose(self.dual, argument.dual, atol=precision))
        )

    def __float__(self):
        return self.real

    def __neg__(self):
        return Variable(-self.real, vars=self.vars, dual=-self.dual)

    def __add__(self, other):
        if isinstance(other, Variable):
            _1 = self._to_dual_type(defaults._global_ad_order)
            _2 = other._to_dual_type(defaults._global_ad_order)
            return _1.__add__(_2)
        elif isinstance(other, (FLOATS, INTS)):
            return Variable(self.real + float(other), vars=self.vars, dual=self.dual)
        elif isinstance(other, Dual):
            _ = Dual(self.real, vars=self.vars, dual=self.dual)
            return _.__add__(other)
        elif isinstance(other, Dual2):
            _ = Dual2(self.real, vars=self.vars, dual=self.dual, dual2=[])
            return _.__add__(other)
        else:
            raise TypeError(f"No operation defined between `Variable` and type: `{type(other)}`")

    def __radd__(self, other):
        return self.__add__(other)

    def __rsub__(self, other):
        return (self.__neg__()).__add__(other)

    def __sub__(self, other):
        return self.__add__(other.__neg__())

    def __mul__(self, other):
        if isinstance(other, Variable):
            _1 = self._to_dual_type(defaults._global_ad_order)
            _2 = other._to_dual_type(defaults._global_ad_order)
            return _1.__mul__(_2)
        elif isinstance(other, (FLOATS, INTS)):
            return Variable(self.real * float(other), vars=self.vars, dual=self.dual * float(other))
        elif isinstance(other, Dual):
            _ = Dual(self.real, vars=self.vars, dual=self.dual)
            return _.__mul__(other)
        elif isinstance(other, Dual2):
            _ = Dual2(self.real, vars=self.vars, dual=self.dual, dual2=[])
            return _.__mul__(other)
        else:
            raise TypeError(f"No operation defined between `Variable` and type: `{type(other)}`")

    def __rmul__(self, other):
        return self.__mul__(other)

    def __truediv__(self, other):
        if isinstance(other, Variable):
            _1 = self._to_dual_type(defaults._global_ad_order)
            _2 = other._to_dual_type(defaults._global_ad_order)
            return _1.__truediv__(_2)
        elif isinstance(other, (FLOATS, INTS)):
            return Variable(self.real / float(other), vars=self.vars, dual=self.dual / float(other))
        elif isinstance(other, Dual):
            _ = Dual(self.real, vars=self.vars, dual=self.dual)
            return _.__truediv__(other)
        elif isinstance(other, Dual2):
            _ = Dual2(self.real, vars=self.vars, dual=self.dual, dual2=[])
            return _.__truediv__(other)
        else:
            raise TypeError(f"No operation defined between `Variable` and type: `{type(other)}`")

    def __rtruediv__(self, other):
        if isinstance(other, Variable):
            # cannot reach this line
            raise TypeError("Impossible line execution - please report issue.")  # pragma: no cover
        elif isinstance(other, (FLOATS, INTS)):
            _1 = Variable(other, ())
            return _1 / self
        elif isinstance(other, Dual):
            _ = Dual(self.real, vars=self.vars, dual=self.dual)
            return other.__truediv__(_)
        elif isinstance(other, Dual2):
            _ = Dual2(self.real, vars=self.vars, dual=self.dual, dual2=[])
            return other.__truediv__(_)
        else:
            raise TypeError(f"No operation defined between `Variable` and type: `{type(other)}`")

    def __exp__(self):
        _1 = self._to_dual_type(defaults._global_ad_order)
        return _1.__exp__()

    def __log__(self):
        _1 = self._to_dual_type(defaults._global_ad_order)
        return _1.__log__()

    def __norm_cdf__(self):
        _1 = self._to_dual_type(defaults._global_ad_order)
        return _1.__norm_cdf__()

    def __norm_inv_cdf__(self):
        _1 = self._to_dual_type(defaults._global_ad_order)
        return _1.__norm_inv_cdf__()

    def __pow__(self, exponent):
        _1 = self._to_dual_type(defaults._global_ad_order)
        return _1.__pow__(exponent)
