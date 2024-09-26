use nalgebra::{SMatrix, SVector};

pub type Value = f32;
pub type TFloatVector<const ROWS: usize> = SVector<Value, ROWS>;
pub type TFloatSquare<const ROWS: usize> = SMatrix<Value, ROWS, ROWS>;
pub type TFloatMatrix<const ROWS: usize, const COLUMNS: usize> = SMatrix<Value, ROWS, COLUMNS>;

pub enum StateTransition<'a, const STATE_ROWS: usize> {
    Matrix(TFloatSquare<STATE_ROWS>),
    TransitionFunction(&'a dyn Fn(f32) -> TFloatSquare<STATE_ROWS>),
}

pub struct KalmanFilter<'a, const STATE_ROWS: usize, const MEASUREMENT_ROWS: usize> {
    state: TFloatVector<STATE_ROWS>,
    error_covariance: TFloatSquare<STATE_ROWS>,
    /// A or fn -> A (state transition matrix)
    a: StateTransition<'a, STATE_ROWS>,
    /// measurement extraction matrix (extracts mesurement from state)
    h: TFloatMatrix<MEASUREMENT_ROWS, STATE_ROWS>,
    /// process noise covariance matrix
    q: TFloatSquare<STATE_ROWS>,
    /// mesurement noise covariance matrix
    r: TFloatSquare<MEASUREMENT_ROWS>,
}

impl<'a, const STATE_ROWS: usize, const MEASUREMENT_ROWS: usize>
    KalmanFilter<'a, STATE_ROWS, MEASUREMENT_ROWS>
{
    ///
    /// Just like in the real Kalman Filter initial state estimate and error error_covariance don't
    /// matter as they are self correcting
    ///
    pub fn new(
        init: TFloatVector<STATE_ROWS>,
        error_covariance: TFloatSquare<STATE_ROWS>,
        stf: StateTransition<'a, STATE_ROWS>,
        h: TFloatMatrix<MEASUREMENT_ROWS, STATE_ROWS>,
        q: TFloatSquare<STATE_ROWS>,
        r: TFloatSquare<MEASUREMENT_ROWS>,
    ) -> Self {
        Self {
            state: init,
            error_covariance,
            a: stf,
            h,
            q,
            r,
        }
    }

    ///
    /// dt: time step
    ///
    pub fn predict(&mut self, dt: f32) {
        let a = match &self.a {
            StateTransition::Matrix(matrix) => matrix,
            StateTransition::TransitionFunction(stf) => &stf(dt),
        };

        // predictions
        self.state = a * self.state;
        self.error_covariance = a * self.error_covariance * a.transpose() + self.q;
    }

    ///
    /// filtering
    ///
    pub fn update(
        &mut self,
        measurement: TFloatVector<MEASUREMENT_ROWS>,
    ) -> Option<TFloatVector<STATE_ROWS>> {
        let hphtr_inv =
            (self.h * self.error_covariance * self.h.transpose() + self.r).try_inverse();

        hphtr_inv?;

        let kalman_gain = self.error_covariance * self.h.transpose() * hphtr_inv.unwrap();

        self.state = self.state + kalman_gain * (measurement - self.h * self.state);
        self.error_covariance =
            self.error_covariance - kalman_gain * self.h * self.error_covariance;

        // return
        Some(self.state)
    }
}
