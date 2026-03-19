use crate::error::PolyError;
use crate::utils;
use ark_ff::Field;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenseMLPoly<F: Field> {
    pub num_vars: usize,
    pub evaluations: Vec<F>,
}

impl<F: Field> DenseMLPoly<F> {
    pub fn new(num_vars: usize, evaluations: Vec<F>) -> Result<Self, PolyError> {
        let expected = 1usize << num_vars;
        if evaluations.len() != expected {
            return Err(PolyError::InvalidLength {
                num_vars,
                expected,
                got: evaluations.len(),
            });
        }
        Ok(Self {
            num_vars,
            evaluations,
        })
    }

    pub fn zero(num_vars: usize) -> Self {
        Self {
            num_vars,
            evaluations: vec![F::zero(); 1 << num_vars],
        }
    }

    pub fn is_zero(&self) -> bool {
        self.evaluations.iter().all(|c| c.is_zero())
    }

    pub fn len(&self) -> usize {
        self.evaluations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.evaluations.is_empty()
    }
}

impl<F: Field> DenseMLPoly<F> {
    pub fn evaluate_inplace(&self, point: &[F]) -> Result<F, PolyError> {
        if point.len() != self.num_vars {
            return Err(PolyError::PointDimensionMismatch {
                expected: self.num_vars,
                got: point.len(),
            });
        }

        let mut buf = self.evaluations.clone();
        let mut size = buf.len();

        for r in point.iter() {
            let half = size / 2;
            for j in 0..half {
                // Write the interpolated value into position j
                buf[j] = (F::one() - r) * buf[j] + *r * buf[j + half];
            }
            size = half;
        }

        Ok(buf[0])
    }
}
