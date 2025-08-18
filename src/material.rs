use crate::color;
use crate::color::Color;
use crate::light::PointLight;
use crate::vector::{Point, Vector};

pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn ambient(&self) -> f64 {
        self.ambient
    }

    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn specular(&self) -> f64 {
        self.specular
    }

    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    pub fn lighting(
        &self,
        light: &PointLight,
        position: &Point,
        eye: &Vector,
        normal: &Vector,
    ) -> Color {
        let effective_color = self.color * *light.intensity();
        let light_vector = (light.position() - position).normalize();

        let ambient = effective_color * self.ambient;

        let light_dot_normal = light_vector.dot(normal);

        // A negative light • normal indicates that the cosine of the angle between the vectors is
        // negative, and the light is behind the surface
        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (color::BLACK, color::BLACK)
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflect = (-light_vector).reflect(normal);
            let reflect_dot_eye = reflect.dot(eye);

            // A negative reflect • eye indicates that the reflection vector points away from the
            // eye vector
            let specular = if reflect_dot_eye < 0.0 {
                color::BLACK
            } else {
                *light.intensity() * self.specular * reflect_dot_eye.powf(self.shininess)
            };

            (diffuse, specular)
        };

        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: color::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::color::Color;
    use crate::light::PointLight;
    use crate::material::Material;
    use crate::vector::{Point, Vector};
    use crate::{color, vector};

    #[test]
    fn test_lighting_light_behind_viewer() {
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), color::WHITE);

        Color::new(1.9, 1.9, 1.9).assert_approx_eq(&Material::default().lighting(
            &light,
            &vector::ORIGIN,
            &eye,
            &normal,
        ));
    }

    #[test]
    fn test_lighting_eye_off_normal() {
        let sqrt_2_2 = 2.0f64.sqrt() / 2.0;

        let eye = Vector::new(0.0, sqrt_2_2, -sqrt_2_2);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), color::WHITE);

        Color::new(1.0, 1.0, 1.0).assert_approx_eq(&Material::default().lighting(
            &light,
            &vector::ORIGIN,
            &eye,
            &normal,
        ));
    }

    #[test]
    fn test_lighting_light_off_normal() {
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), color::WHITE);

        Color::new(0.7364, 0.7364, 0.7364).assert_approx_eq_epsilon(
            &Material::default().lighting(&light, &vector::ORIGIN, &eye, &normal),
            1e-4,
        );
    }

    #[test]
    fn test_lighting_eye_and_light_off_normal() {
        let sqrt_2_2 = 2.0f64.sqrt() / 2.0;

        let eye = Vector::new(0.0, -sqrt_2_2, -sqrt_2_2);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), color::WHITE);

        Color::new(1.6364, 1.6364, 1.6364).assert_approx_eq_epsilon(
            &Material::default().lighting(&light, &vector::ORIGIN, &eye, &normal),
            1e-4,
        );
    }

    #[test]
    fn test_lighting_light_behind_surface() {
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), color::WHITE);

        Color::new(0.1, 0.1, 0.1).assert_approx_eq(&Material::default().lighting(
            &light,
            &vector::ORIGIN,
            &eye,
            &normal,
        ));
    }
}
