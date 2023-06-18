use ndarray::{arr1, Array1};

const GRAVITY: f64 = 0.05;
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub fart: Array1<f64>,
}

fn vektor_lengde(vektor: &Array1<f64>) -> f64 {
    return (vektor[0] * vektor[0] + vektor[1] * vektor[1]).powf(1.0 / 2.0);
}

impl Particle {
    pub fn new(x: f64, y: f64, x_vel: f64, y_vel: f64) -> Self {
        return Self {
            x: x,
            y: y,
            fart: arr1(&[x_vel, y_vel]),
        };
    }

    pub fn apply_physics(&mut self, delta_time: f64, origo: &Array1<f64>) {
        let posisjon: Array1<f64> = arr1(&[self.x, self.y]);

        let vektor_til_origo: Array1<f64> = origo - posisjon;

        let avstand_til_origo: f64 = vektor_lengde(&vektor_til_origo);

        if avstand_til_origo > 0.5 {
            let mut akselerasjon_mot_origo: Array1<f64> =
                vektor_til_origo * (GRAVITY / avstand_til_origo.powf(2.0)) * delta_time;

            let mut view1 = akselerasjon_mot_origo.view_mut();
            self.fart
                .zip_mut_with(&mut view1, |a: &mut f64, b: &f64| *a += *b);
        }

        self.x += self.fart[0];
        self.y += self.fart[1];
    }
}
