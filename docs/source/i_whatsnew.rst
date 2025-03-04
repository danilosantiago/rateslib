.. _whatsnew-doc:

.. role:: red

**************
Release Notes
**************

The future development of *rateslib* is open to many avenues.
Some possibilities are listed below. The author is very interested in any feedback
and this can be given on the public **Issues** board at the project github
repository: `Rateslib Project <https://github.com/attack68/rateslib>`_, or by direct
email contact, see `rateslib <https://rateslib.com>`_.

1.6.0 (No release date)
****************************

.. list-table::
   :widths: 25 75
   :header-rows: 1

   * - Feature
     - Description
   * - Instruments
     - Add :class:`~rateslib.instruments.CDS` for credit pricing, as well as the associated components;
       :class:`~rateslib.legs.CreditPremiumLeg`, :class:`~rateslib.periods.CreditPremiumPeriod`,
       :class:`~rateslib.legs.CreditProtectionLeg`, :class:`~rateslib.periods.CreditProtectionPeriod`.
       (`419 <https://github.com/attack68/rateslib/pull/419>`_)
       (`425 <https://github.com/attack68/rateslib/pull/425>`_)
       (`426 <https://github.com/attack68/rateslib/pull/426>`_)
   * - Instruments
     - Add an additional method :meth:`~rateslib.instruments.CDS.analytic_rec_risk` to measure the
       sensitivity of a change in ``recovery_rate`` for a :class:`~rateslib.instruments.CDS`.
       (`448 <https://github.com/attack68/rateslib/pull/448>`_)
   * - Instruments
     - Add the ``spec`` options; *'audusd_xcs'*, *'audusd_xcs3'*, *'nzdusd_xcs3'*, *'nzdaud_xcs3'*,
       *'us_ig_cds'*
       (`429 <https://github.com/attack68/rateslib/pull/429>`_)
       (`454 <https://github.com/attack68/rateslib/pull/454>`_)
   * - Performance
     - *Curve caching* introduced to :class:`~rateslib.curves.Curve`, :class:`~rateslib.curves.LineCurve`,
       :class:`~rateslib.curves.IndexCurve` to improve performance of repeatedly fetched curve values such as
       in *Solvers* and standardised *Instruments*. This feature can be opted out of using the
       ``defaults.curve_caching`` setting. Note also the added :meth:`~rateslib.curves.Curve.clear_cache` method.
       (`435 <https://github.com/attack68/rateslib/pull/435>`_)
   * - Automatic Differentiation
     - Add a new object for AD management, a :class:`~rateslib.dual.Variable`, which allows a
       user to inject manual exogenous sensitivities into calculations. See
       :ref:`what is an exogenous Variable? <cook-exogenous-doc>`
       (`452 <https://github.com/attack68/rateslib/pull/452>`_)
   * - Risk Sensitivities
     - Add method :meth:`~rateslib.instruments.Sensitivities.exo_delta` to calculate the delta
       sensitivity against a user-defined exogenous *Variable*.
       (`453 <https://github.com/attack68/rateslib/pull/453>`_)
   * - Bug
     - :class:`~rateslib.curves.MultiCsaCurve` is now included in the main namespace.
       (`436 <https://github.com/attack68/rateslib/pull/436>`_)
   * - Bug
     - Adding *Dual* or *Dual2* type ``spread`` using :meth:`~rateslib.curves.Curve.shift` method
       now avoids *TypeErrors* where possible and maintains appropriate AD orders for each
       existing and new object.
       (`440 <https://github.com/attack68/rateslib/pull/440>`_)
   * - Developers
     - *rateslib-rs* extension upgrades to using PyO3:0.22, nadarray:0.16, numpy:0.22.
       (`460 <https://github.com/attack68/rateslib/pull/460>`_)

1.5.0 (25th September 2024)
****************************

.. list-table::
   :widths: 25 75
   :header-rows: 1

   * - Feature
     - Description
   * - Instruments
     - Added *"nzd_irs3"*, *"nzd_irs6"*, *"se_gbb"* and *"uk_gbb"* to available ``spec`` defaults.
       (`397 <https://github.com/attack68/rateslib/pull/397>`_)
       (`403 <https://github.com/attack68/rateslib/pull/403>`_)
   * - Instruments
     - :class:`~rateslib.instruments.BondCalcMode` and :class:`~rateslib.instruments.BillCalcMode`
       added to allow more flexibility when adding new bond specifications with other
       defined calculation conventions.
       (`402 <https://github.com/attack68/rateslib/pull/402>`_)
   * - Calendars
     - Add a *"wlg"* calendar for New Zealand *IRS*.
       (`363 <https://github.com/attack68/rateslib/pull/363>`_)
   * - Calendars
     - Add a method, :meth:`~rateslib.calendars.get_imm`, to calculate IMM dates.
       `(371) <https://github.com/attack68/rateslib/pull/371>`_
   * - Serialization
     - *PPSplines* are now serializable. Read more :ref:`here <serialization-doc>`.
       `(374) <https://github.com/attack68/rateslib/pull/374>`_
   * - Refactor
     - :red:`Minor Breaking Change!` *PPSpline* equality is now *True* if both spline
       coefficients are unsolved, i.e. *None*.
       `(374) <https://github.com/attack68/rateslib/pull/374>`_
   * - Refactor
     - The ``__repr__`` method of all *Curve* types, *FXRates* and *FXForwards* types, the *Solver*, *Schedule*,
       and all *Period*, *Leg* and *Instrument* types are changed for better display in associated
       packages.
       `(387) <https://github.com/attack68/rateslib/pull/387>`_
       `(388) <https://github.com/attack68/rateslib/pull/388>`_
       `(389) <https://github.com/attack68/rateslib/pull/389>`_
       `(390) <https://github.com/attack68/rateslib/pull/390>`_
       `(413) <https://github.com/attack68/rateslib/pull/413>`_
       `(416) <https://github.com/attack68/rateslib/pull/416>`_
       `(418) <https://github.com/attack68/rateslib/pull/418>`_
   * - Performance
     - Improve the speed of bond :meth:`~rateslib.instruments.FixedRateBond.ytm` calculations from about 750us to
       500us on average.
       `(380) <https://github.com/attack68/rateslib/pull/380>`_
   * - Bug
     - :class:`~rateslib.fx.FXRates` fix support for pickling which allows multithreading across CPU pools or
       external serialization.
       `(393) <https://github.com/attack68/rateslib/pull/393>`_
   * - Bug
     - The ``eom`` parameter for spec *"us_gb"* and *"us_gb_tsy"* and associated aliases is corrected to *True*.
       `(368) <https://github.com/attack68/rateslib/pull/368>`_
   * - Bug
     - Creating *IRS* or similar *Instruments* with a ``termination`` of "1b" or business days
       now correctly uses the specified calendar.
       `(378) <https://github.com/attack68/rateslib/pull/378>`_
   * - Bug
     - :class:`~rateslib.curves.ProxyCurve`, :class:`~rateslib.curves.CompositeCurve`, and
       :class:`~rateslib.curves.MultiCsaCurve` now correctly initialise a randomised curve ``id``
       when one is not provided.
       `(387) <https://github.com/attack68/rateslib/pull/387>`_
   * - Bug
     - Altered the *default specs* for ``eur_stir3`` to reflect a EURIBOR settlement, and
       ``aud_irs3`` to reflect a no-lagged publication.
       `(395) <https://github.com/attack68/rateslib/pull/395>`_
   * - Bug
     - The conventions for *"SE_GBB"* and *"SE_GB"* amended for
       T+2 settle instead of T+1, and the calculation for YTM adjusted for simple yield in the
       last coupon period.
       `(410) <https://github.com/attack68/rateslib/pull/410>`_
   * - Bug
     - IMM FRAs with an IMM roll date only need to define the IMM ``roll`` on leg1 and no longer
       also on leg2.
       `(409) <https://github.com/attack68/rateslib/pull/409>`_


1.4.0 (28th Aug 2024)
***********************

.. list-table::
   :widths: 25 75
   :header-rows: 1

   * - Feature
     - Description
   * - Calendars
     - :meth:`~rateslib.calendars.add_tenor` acquires the new optional argument ``mod_days`` which, by
       default, negates the modification rule for day type tenors and applies it only to month and year type tenors.
   * - Calendars
     - Add :class:`~rateslib.calendars.NamedCal` for improved control of calendar serialization and loading.
   * - Instruments
     - Add a :meth:`~rateslib.instruments.FXOption.cashflows` method to generic :class:`~rateslib.instruments.FXOption`
       and also as a pre-requisite to :class:`~rateslib.periods.FXOptionPeriod`. This also allows the derivative
       method :meth:`~rateslib.instruments.Sensitivities.cashflows_table` to function for *FXOption*.
   * - Instruments
     - Add an internal routine to derive *FXOption* `expiry` and `delivery` according to FX market conventions using
       the new settlement calendar system introduced in v1.3.0.
   * - Instruments
     - Add ``eom`` parameter to *FXOptions* for exact expiry and delivery date calculation when given as string tenor.
   * - Instruments
     - The default ``calc_mode`` for *Bill*, *FixedRateBond*, *FloatRateNote* and *IndexFixedRateBond* is now
       separately configurable for each type.
   * - Instruments / Legs
     - Can now have *effective* and *termination* dates which are non-business dates
       in unmodified schedules.
   * - Surfaces
     - Add ``weights`` to :class:`~rateslib.fx_volatility.FXDeltaVolSurface` to give more control of temporal
       interpolation of volatility.
   * - Bug
     - Publicly exposed the :meth:`PPSpline.bsplmatrix <rateslib.splines.PPSplineF64.bsplmatrix>` function
       for displaying intermediate spline calculation results of the spline coefficient matrix.
   * - Bug
     - *Dual* and *Dual2* fix support for pickling which allows multithreading across CPU pools.
   * - Bug
     - Expose :meth:`~rateslib.dual.gradient` as a method in the *rateslib* public API.
   * - Bug
     - Expose :class:`~rateslib.calendars.NamedCal` as a class in the *rateslib* public API.
   * - Bug
     - :class:`~rateslib.instruments.IndexFixedRateBond` now correctly initialises when using a
       :class:`pandas.Series` as ``index_fixings`` argument.
   * - Bug
     - :class:`~rateslib.instruments.ZCIS` now raises if an ``index_base`` cannot be forecast from an *IndexCurve*
       and the value should be known and input directly, to avoid *Solver* calibration failures.
   * - Bug
     - ``npv`` and ``cashflows`` of a :class:`~rateslib.periods.FloatPeriod` now handle
       error messages regarding missing RFR fixings for an historical period which is only
       missing a single fixing.

1.3.0 (9th July 2024)
***********************

.. list-table::
   :widths: 25 75
   :header-rows: 1

   * - Feature
     - Description
   * - Instruments
     - ``calc_mode`` of :class:`~rateslib.instruments.FixedRateBond` has been refactored to allow more standardised
       names. The existing modes are deprecated and will be removed in v2.0.
   * - Instruments
     - ``spec`` *"de_gb"*, *"fr_gb"*, *"it_gb"*, *"no_gb"* and *"nl_gb"*,
       added to :class:`~rateslib.instruments.FixedRateBond` to quickly create German, French,
       Italian, Norwegian and Dutch government bonds.
   * - Calendars
     - The `pandas` holiday and calendar system has been removed in favour of a rust implementation for
       calendar objects: :class:`~rateslib.calendars.Cal` and :class:`~rateslib.calendars.UnionCal`.
   * - Calendars
     - :red:`Breaking Change!` The :meth:`~rateslib.calendars.create_calendar` methods is deprecated and
       modified to accept different input arguments.
   * - Calendars
     - Calendar string parsing has been enhanced to allow associated settlement calendars, and
       automatic creation of a :class:`~rateslib.calendars.UnionCal` object. E.g. *"tgt,ldn|nyc"*.
   * - Calendars
     - The Tokyo calendar *'tyo'* has been added to align with TONA publication. The FED calendar *'fed'* has also been
       added. The Sydney calendar *"syd"* has been added to align with AONIA publication.
   * - Calendars
     - JSON serialisation/deserialisation of :class:`~rateslib.calendars.Cal`
       and :class:`~rateslib.calendars.UnionCal` added for saving/loading from database or file.
   * - Calendars
     - The new DCF method *'Bus252'* is added to allow Brazilian type calculations.
   * - Dual
     - JSON serialisation/deserialisation of :class:`~rateslib.dual.Dual`
       and :class:`~rateslib.dual.Dual2` added for saving/loading from database or file.
   * - FXRates
     - The :class:`~rateslib.fx.FXRates` class has been delegated to the Rust extension to improve performance.
   * - Performance
     - Algorithm for :class:`~rateslib.fx.FXRates` generation is modified to improve the speed of instance
       construction for a larger number of currencies.
   * - FX Volatility
     - :meth:`~rateslib.fx_volatility.FXDeltaVolSmile.get_from_strike` on both *Smiles* and *Surfaces* has
       been refactored to remove the unnecessary ``phi`` argument.
   * - Bug
     - :class:`~rateslib.instruments.ZCS` now raises if fixed frequency is given as "Z".
   * - Bug
     - :meth:`~rateslib.instruments.FixedRateBond.rate` method of a *FixedRateBond* now correctly
       returns the local currency price or yield-to-maturity without being wrongly converted by a
       ``base`` FX rate, if an FX object is also supplied to the pricing formula.
   * - Bug
     - :class:`~rateslib.instruments.FXOption` initialised with ``metric`` no longer
       raises if an alternate dynamic ``metric`` is requested as override in the
       :meth:`~rateslib.instruments.FXOption.rate` method.
   * - Bug
     - Setting and resetting some types of values (namely by-reference stored values) of the ``defaults`` object
       is no longer ineffective.
   * - Bug
     - Solving acyclic *FXForwards* systems is now stable for all orderings of currencies, and does not depend
       on a well chosen ``base`` currency.
   * - Bug
     - Converting an `fx_array` associated with the :class:`~rateslib.fx.FXRates` into second order for AD
       calculations now captures second order FX derivatives correctly by rebuilding the array, instead of a
       direct conversion setting second order derivatives to zero.
   * - Bug
     - Entering the *"single_vol"* ``metric`` into the :meth:`~rateslib.instruments.FXBrokerFly.rate` method
       of a :class:`~rateslib.instruments.FXBrokerFly` no longer raises.
   * - Errors
     - Improved messages when missing `fx` objects for pricing :class:`~rateslib.instruments.FXExchange`.


1.2.2 (31st May 2024)
**********************

This version uses **Rust** bindings. See :ref:`getting started <pricing-doc>`
for notes about installation changes.

New *FX Volatility Products* are set to **beta** status, probably until version 2.0.

.. list-table::
   :widths: 25 75
   :header-rows: 1

   * - Feature
     - Description
   * - Performance
     - The modules ``rateslib.dual`` and ``rateslib.splines`` have been ported to **Rust**
       instead of Python to improve calculation times.
   * - Splines
     - New methods :meth:`~rateslib.splines.PPSplineF64.ppev_single_dual`,
       :meth:`~rateslib.splines.PPSplineF64.ppev_single_dual2`,
       :meth:`~rateslib.splines.PPSplineF64.ppdnev_single_dual`,
       and :meth:`~rateslib.splines.PPSplineF64.ppdnev_single_dual2` have been added to
       ensure correct handling of AD with regards to both x-axis and y-axis variables. See
       :ref:`section on using AD with splines <splines-ad-doc>`
   * - Splines
     - Added :meth:`~rateslib.splines.evaluate` for automatically handling which *ppdnev* method
       to use based on the AD sensitivities of the given `x` value.
   * - Instruments
     - :red:`Breaking Changes!` Amend :class:`~rateslib.instruments.FXExchange` to **remove** the
       arguments ``currency`` and ``leg2_currency``
       in favour of using ``pair`` which is consistent with the new *FX Volatility* naming convention.
       Also **reverse** the ``notional`` so that a +1mm EURUSD transaction is considered as a purchase of
       EUR and a sale of USD.
   * - Instruments
     - :class:`~rateslib.instruments.FXSwap` allows the dominant ``pair`` argument, consistent with other *FX*
       instruments to define the currencies. ``currency`` and ``leg2_currency`` are still currently permissible if
       ``pair`` is omitted.
   * - Instruments
     - Basic *FX Volatility Instruments* have been added in **beta** status, including
       :class:`~rateslib.instruments.FXCall`, :class:`~rateslib.instruments.FXPut`,
       :class:`~rateslib.instruments.FXRiskReversal`, :class:`~rateslib.instruments.FXStraddle`,
       :class:`~rateslib.instruments.FXStrangle`, :class:`~rateslib.instruments.FXBrokerFly`
       and :class:`~rateslib.instruments.FXOptionStrat`.
       See :ref:`user guide section <fx-volatility-doc>` for more information.
   * - FX Volatility
     - New pricing components :class:`~rateslib.fx_volatility.FXDeltaVolSmile` and
       :class:`~rateslib.fx_volatility.FXDeltaVolSurface`
       have been added
       to allow pricing of single expiry *FX Options* with a *Smile* interpolated over a *Delta*
       axis. See :ref:`FX volatility construction <c-fx-smile-doc>`.
   * - AD
     - Added :meth:`~rateslib.dual.dual_norm_pdf` for AD safe standard normal probability density.
   * - AD
     - Added :meth:`~rateslib.solver.newton_1dim` and :meth:`~rateslib.solver.newton_ndim`
       for AD safe Newton root solving in one or multiple dimensions.
   * - Solver
     - Added :meth:`~rateslib.solver.quadratic_eqn` to return the solution of a quadratic equation
       in an AD safe and consistent return format to other solvers for convenience.
   * - Bug
     - "ActActICMA" convention now handles ``frequency`` of "Z", asserting that of "A",
       albeit with a *UserWarning*.
   * - Bug
     - ``npv`` and ``cashflows`` of a :class:`~rateslib.periods.FloatPeriod` did not
       handle error messages regarding missing RFR fixings for a historical period.
       Calculations wll now raise if missing ``fixings``.
   * - Bug
     - `FXSwap` now no longer raises `TypeError` for dual number type mixing when `npv` or `rate`
       are called after changing the AD order of curves and fx objects.


1.1.0 (20th Mar 2024)
**********************

.. list-table::
   :widths: 25 75
   :header-rows: 1

   * - Feature
     - Description
   * - Automatic Differentiation
     - :red:`Breaking Change!` Dual number `gradient` method is no longer calculable on the object.
       Instead of `dual.gradient(vars)` use the following call `gradient(dual, vars)`, using the
       provided function :meth:`rateslib.dual.gradient`.
   * - Instruments
     - Added argument ``metric`` to :class:`~rateslib.instruments.Value` so that specific *Curve* values derived
       as calculated figures (e.g. continuously compounded zero rate, or index value) can be calibrated by *Solvers*.
   * - Bug
     - :meth:`~rateslib.solver.Solver.delta` and :meth:`~rateslib.solver.Solver.gamma` now work directly with
       given ``npv`` when ``fx`` is not provided.
   * - Bug
     - :meth:`~rateslib.periods.FloatPeriod.npv` now returns 0.0 for historical payment dates correctly when
       given the ``local`` argument.
   * - Bug
     - :meth:`~rateslib.periods.IndexCashflow.cashflows` no longer prints dual numbers to tables.
   * - Performance
     - Curve iterations in the :class:`~rateslib.solver.Solver` were amended in the way they handle
       :class:`~rateslib.dual.Dual` variables in order to reduce upcasting and increase the speed of basic operations.
   * - Performance
     - :class:`~rateslib.splines.bsplev_single` introduced a short circuit based on the positivity and support
       property to greatly improve time needed to solve curves with splines.
   * - Performance
     - :class:`~rateslib.curves.Curve` with splines are remapped to use float posix timestamps rather than datetimes
       for building splines. Operations with floats are much faster than their equivalents using timedeltas.


1.0.0 (1st Feb 2024)
**********************

.. list-table::
   :widths: 25 75
   :header-rows: 1

   * - Feature
     - Description
   * - Bug
     - :meth:`~rateslib.instruments.FRA.cashflows` now correctly identifies the DF at cash
       settled payment date.
   * - Bug
     - :meth:`~rateslib.legs.FloatLeg.fixings_table` now generates exact results (not in approximate mode) when RFR
       fixings are included in any period.


0.7.0 (29th Nov 2023)
**********************

.. list-table::
   :widths: 25 75
   :header-rows: 1

   * - Feature
     - Description
   * - Legs
     - Refactor how the ``defaults.fixings`` object works. **Breaking change**. Explained in
       :ref:`Working with Fixings <cook-fixings-doc>`.
   * - Legs
     - Allow ``fixings`` as a 2-tuple to manually define the first *FloatPeriod* (say as IBOR stub)
       and determine the rest from a *Series*. Also allow ``fx_fixings`` as a 2-tuple for similar
       reason for MTM *XCS*.
   * - Instruments
     - :class:`~rateslib.instruments.Fly` and :class:`~rateslib.instruments.Spread` now express
       *rate* in basis point terms and not percent.
   * - Instruments
     - Added ``calc_mode`` to :class:`~rateslib.instruments.BondFuture` to calculate CME US treasury
       conversion factors correctly.
   * - Instruments
     - :class:`~rateslib.instruments.BondFuture.ctd_index` can now optionally return the ordered set of CTD indexes
       instead of just the CTD.
   * - Instruments
     - Added :meth:`~rateslib.instruments.BondFuture.cms` to perform multi-security CTD analysis on
       :class:`~rateslib.instruments.BondFuture`.
   * - Solver
     - Add an attribute ``result`` that contains retrievable iteration success or failure
       information.
   * - Bug
     - Update :meth:`~rateslib.instruments.STIRFuture.analytic_delta` for
       :class:`~rateslib.instruments.STIRFuture` to match *delta*.
   * - Bug
     - Add the ``spec`` argument functionality missing for
       :class:`~rateslib.instruments.IndexFixedRateBond`.
   * - Bug
     - :class:`~rateslib.curves.CompositeCurve` now returns zero for DF item lookups prior to the initial node date.
   * - Bug
     - :class:`~rateslib.instruments.BondFuture.net_basis` now deducts accrued from the result when the prices are
       provided ``dirty``.

0.6.0 (19th Oct 2023)
**********************

.. list-table::
   :widths: 25 75
   :header-rows: 1

   * - Feature
     - Description
   * - Instruments
     - Add a :class:`~rateslib.instruments.STIRFuture` class
   * - Instruments
     - Merge all :class:`~rateslib.instruments.XCS` classes into one, adding new arguments,
       ``fixed``, ``leg2_fixed`` and ``leg2_mtm`` to differentiate between types.
   * - Curves
     - Separate :class:`~rateslib.curves.MultiCsaCurve`
       from :class:`~rateslib.curves.CompositeCurve` for increased transparency on its action.
   * - Curves
     - Add the ability to supply curves in a dict for forecasting *FloatPeriods* to be
       able handle interpolated stub periods under an *"ibor"* ``fixing_method``.
   * - Solver
     - Added the methods :meth:`~rateslib.solver.Solver.jacobian` and
       :meth:`~rateslib.solver.Solver.market_movements` for coordinating multiple *Solvers*.
   * - Bug
     - Instrument ``spec`` with ``method_param`` set to 2 day lag for certain IBOR instruments.
   * - Bug
     - The :meth:`~rateslib.instruments.Portfolio.npv` method on a *Portfolio* no longer allows
       mixed currency outputs to be aggregated into a single float value.
   * - Bug
     - Now emit a warning if a discount factor or rate is requested on a curve with a spline
       outside of the rightmost boundary of the spline interval.


0.5.1 (11 Sep 2023)
**********************

.. list-table::
   :widths: 25 75
   :header-rows: 1

   * - Feature
     - Description
   * - Instruments
     - Rename :class:`~rateslib.instruments.FloatRateBond`
       to :class:`~rateslib.instruments.FloatRateNote` and removed the
       alias :class:`~rateslib.instruments.Swap`.
   * - Instruments
     - Add a ``spec`` keyword argument to allow instruments to be pre-defined and follow
       market conventions without the user needing to input these directly, but preserving an
       ability to overwrite specific values.
   * - Instruments
     - Add ``calc_mode`` to *Bonds* to provide mechanisms to perform YTM calculations under
       different conventions and geographies.
   * - Periods
     - :class:`~rateslib.periods.FloatPeriod` now allows **averaging** methods for
       determining the rate.
   * - Curves
     - The :meth:`shift()<rateslib.curves.Curve.shift>` operation for *Curves* now defaults to using
       a *CompositeCurve* approach to preserve a constant spread to the underlying *Curve* via
       a dynamic association. Shifted curves can also optionally add ``id`` and ``collateral``
       tags.
   * - Schedule
     - A :class:`~rateslib.scheduling.Schedule` now has the arguments ``eval_date`` and
       ``eval_mode`` allow a tenor-tenor effective-termination input.
   * - Defaults
     - Change the default :class:`~rateslib.solver.Solver` algorithm to *"levenberg_marquardt"*
       because it is more robust for new users, even if slower in general.
   * - Bug
     - :class:`~rateslib.instruments.FXExchange` can now be imported from *rateslib* and has been added
       to ``__init__``.
   * - Bug
     - :meth:`~rateslib.instruments.Sensitivities.cashflows_table` no longer returns empty when
       no collateral information is available.
   * - Bug
     - :meth:`~rateslib.periods.FloatPeriod.fixings_table` now properly represents published
       fixing values as having zero nominal exposure.
   * - Bug
     - ``solver.fx`` attribute is now properly passed through to the ``rate`` calculation
       of multi-currency instruments when ``fx`` is *None*.


0.4.0 (12 Aug 2023)
********************

.. list-table::
   :widths: 25 75
   :header-rows: 1

   * - Feature
     - Description
   * - Instruments
     - Added ``split_notional`` to :class:`~rateslib.instruments.FXSwap` to more accurately
       reflect the interbank traded product.
   * - Instruments
     - Added :class:`~rateslib.instruments.FXExchange`, to provide booking FX spot or FX forward
       trades.
   * - Legs
     - Removed all ``LegExchange`` types, and replaced by adding ``initial_exchange`` and
       ``final_exchange`` as arguments to basic ``Legs``.
   * - Instruments
     - The ``payment_lag_exchange`` parameter for ``FXSwap`` was removed in favour of using
       ``payment_lag``.
   * - Defaults
     - Added historic fixing data until end July for ESTR, SOFR,
       SWESTR, SONIA and NOWA, for testing and validation.
   * - Instruments
     - Collateral tags were added to *Curves* to permit the new method ``cashflows_table`` which
       tabulates future cashflows according to currency and collateral type.
   * - Performance
     - Calendars are now cached which improves general performance by about 10%.
   * - Bug
     - When performing operations on *CompositeCurves* the resultant curve now correctly inherits
       the ``multi_csa`` parameters.
   * - Bug
     - ``FloatPeriod`` fixing exposure tables were marginally overestimated by ignoring
       discounting effects. This is corrected.
   * - Bug
     - NumPy.float128 datatype is not available on Windows and caused loading errors.
   * - Bug
     - The holiday calendars: 'ldn', 'tgt', 'nyc', 'stk', 'osl', and 'zur', have been reviewed
       and validated historic fixings against the historic fixing data. These are also now
       fully documented.
   * - Bug
     - *CompositeCurve* can now be constructed from *ProxyCurve* and *Curve* combinations.


0.3.1 (29 Jul 2023)
*********************

.. list-table::
   :widths: 25 75
   :header-rows: 1

   * - Feature
     - Description
   * - Legs
     - Added :class:`~rateslib.legs.IndexFixedLeg`,
       :class:`~rateslib.legs.ZeroIndexLeg`,
       and :class:`~rateslib.legs.IndexFixedLegExchange`.
   * - Instruments
     - Added :class:`~rateslib.instruments.IndexFixedRateBond`,
       :class:`~rateslib.instruments.IIRS`, :class:`~rateslib.instruments.ZCIS`.
   * - Curves
     - Added :class:`~rateslib.curves.CompositeCurve`.

0.2.0 (15 May 2023)
**********************

.. list-table::
   :widths: 25 75
   :header-rows: 1

   * - Feature
     - Description
   * - Instruments
     - Added :class:`~rateslib.instruments.BondFuture`.
   * - Curves
     - Added :class:`~rateslib.curves.IndexCurve`.

0.1.0 (24 Apr 2023)
**********************

.. list-table::
   :widths: 25 75
   :header-rows: 1

   * - Feature
     - Description
   * - Automatic Differentiation
     - A toolset for making risk sensitivity and gradient based calculations.
   * - Calendars
     - A toolset for handling dates and holiday calendars for schedules.
   * - Schedule
     - A toolset for generating financial schedules of financial instruments.
   * - Splines
     - A toolset for allowing spline interpolation.
   * - Curves
     - Initial classes for DF bases and value based interest rate curves.
   * - Periods
     - Initial classes for handling fixed periods, float periods and cashflows.
   * - Legs
     - Initial classes for aggregating periods.
   * - Instruments
     - Adding standard financial instruments such as securities: bonds and bills,
       and derivatives such as: IRS, SBS, FRA, XCS, FXSwap
   * - Solver
     - A set of algorithms for iteratively determining interest rate curves.
   * - FX
     - Initial classes for handling FX rates an Fx forwards.
