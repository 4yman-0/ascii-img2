use image::GenericImageView;

pub struct Line<'a, T: GenericImageView> {
    image: &'a T,
    x: u32,
    y: u32,
    width: u32,
}

impl<'a, T: GenericImageView> Iterator for Line<'a, T> {
    type Item = T::Pixel;
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.width {
            None
        } else {
            let pixel = self.image.get_pixel(self.x, self.y);
            self.x += 1;
            Some(pixel)
        }
    }
}

pub struct Lines<'a, T: GenericImageView> {
    image: &'a T,
    y: u32,
    width: u32,
    height: u32,
}

impl<'a, T: GenericImageView> Iterator for Lines<'a, T> {
    type Item = Line<'a, T>;
    fn next(&mut self) -> Option<Line<'a, T>> {
        if self.y >= self.height {
            None
        } else {
            let line = Line {
                image: self.image,
                x: 0,
                y: self.y,
                width: self.width,
            };
            self.y += 1;
            Some(line)
        }
    }
}

pub trait LinesTrait<'a>: GenericImageView + Sized {
    fn lines(&'a self) -> Lines<'a, Self>;
}

impl<'a, T: GenericImageView + Sized> LinesTrait<'a> for T {
    fn lines(&'a self) -> Lines<'a, Self> {
        let (width, height) = self.dimensions();
        Lines {
            image: self,
            width,
            height,
            y: 0,
        }
    }
}
