use std::io::{Read, Write};

use byteorder::{ReadBytesExt, LittleEndian, WriteBytesExt};
use record::EsriShape;
use ShapeType;
use std::mem::size_of;

use super::Error;
use record::{is_no_data, ReadableShape, BBox};

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl ReadableShape for Point {
    type ActualShape = Self;

    fn shapetype() -> ShapeType {
        ShapeType::Point
    }

    fn read_from<T: Read>(source: &mut T) -> Result<Self::ActualShape, Error> {
        let x = source.read_f64::<LittleEndian>()?;
        let y = source.read_f64::<LittleEndian>()?;
        Ok(Self { x, y })
    }
}


impl EsriShape for Point {
    fn shapetype(&self) -> ShapeType {
        ShapeType::Point
    }

    fn size_in_bytes(&self) -> usize {
        2 * size_of::<f64>()
    }

    fn write_to<T: Write>(self, dest: &mut T) -> Result<(), Error> {
        dest.write_f64::<LittleEndian>(self.x)?;
        dest.write_f64::<LittleEndian>(self.y)?;
        Ok(())
    }

    fn bbox(&self) -> BBox {
        BBox {
            xmin: self.x,
            ymin: self.y,
            xmax: self.x,
            ymax: self.y,
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

pub struct PointM {
    pub x: f64,
    pub y: f64,
    pub m: f64,
}


impl ReadableShape for PointM {
    type ActualShape = Self;

    fn shapetype() -> ShapeType {
        ShapeType::PointM
    }

    fn read_from<T: Read>(mut source: &mut T) -> Result<Self::ActualShape, Error> {
        let point = Point::read_from(&mut source)?;
        let m = source.read_f64::<LittleEndian>()?;
        Ok(Self {
            x: point.x,
            y: point.y,
            m,
        })
    }
}

impl EsriShape for PointM {
    fn shapetype(&self) -> ShapeType {
        ShapeType::PointM
    }

    fn size_in_bytes(&self) -> usize {
        3 * size_of::<f64>()
    }

    fn write_to<T: Write>(self, dest: &mut T) -> Result<(), Error> {
        dest.write_f64::<LittleEndian>(self.x)?;
        dest.write_f64::<LittleEndian>(self.y)?;
        dest.write_f64::<LittleEndian>(self.m)?;
        Ok(())
    }

    fn bbox(&self) -> BBox {
        BBox {
            xmin: self.x,
            ymin: self.y,
            xmax: self.x,
            ymax: self.y,
        }
    }

    fn m_range(&self) -> [f64; 2] {
        if is_no_data(self.m) {
            [0.0, 0.0]
        } else {
            [self.m, self.m]
        }
    }
}


pub struct PointZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub m: f64,
}

impl ReadableShape for PointZ {
    type ActualShape = Self;

    fn shapetype() -> ShapeType {
        ShapeType::PointZ
    }

    fn read_from<T: Read>(mut source: &mut T) -> Result<Self::ActualShape, Error> {
        let point = Point::read_from(&mut source)?;
        let z = source.read_f64::<LittleEndian>()?;
        let m = source.read_f64::<LittleEndian>()?;
        Ok(Self {
            x: point.x,
            y: point.y,
            z,
            m,
        })
    }
}

impl EsriShape for PointZ {
    fn shapetype(&self) -> ShapeType {
        ShapeType::PointZ
    }

    fn size_in_bytes(&self) -> usize {
        4 * size_of::<f64>()
    }

    fn write_to<T: Write>(self, dest: &mut T) -> Result<(), Error> {
        dest.write_f64::<LittleEndian>(self.x)?;
        dest.write_f64::<LittleEndian>(self.y)?;
        dest.write_f64::<LittleEndian>(self.z)?;
        dest.write_f64::<LittleEndian>(self.m)?;
        Ok(())
    }

    fn bbox(&self) -> BBox {
        BBox {
            xmin: self.x,
            ymin: self.y,
            xmax: self.x,
            ymax: self.y,
        }
    }

    fn z_range(&self) -> [f64; 2] {
        [self.z, self.z]
    }

    fn m_range(&self) -> [f64; 2] {
        if is_no_data(self.m) {
            [0.0, 0.0]
        } else {
            [self.m, self.m]
        }
    }
}