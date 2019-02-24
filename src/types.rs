use crate::sql_types::*;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use postgis::ewkb::{MultiPolygonT, Point, PolygonT};
use std::convert::From;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[sql_type = "Geography"]
pub struct GeogPoint {
    pub x: f64, // lon
    pub y: f64, // lat
    pub srid: Option<i32>,
}

impl From<Point> for GeogPoint {
    fn from(p: Point) -> Self {
        let Point { x, y, srid } = p;
        Self { x, y, srid }
    }
}
impl From<GeogPoint> for Point {
    fn from(p: GeogPoint) -> Self {
        let GeogPoint { x, y, srid } = p;
        Self { x, y, srid }
    }
}

impl FromSql<Geography, Pg> for GeogPoint {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        use postgis::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        Ok(Point::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<Geography, Pg> for GeogPoint {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
        Point::from(*self).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[sql_type = "Geography"]
pub struct LineString {
    pub points: Vec<LineStringT<P>>,
    pub srid: Option<i32>,
}

impl From<PolygonT> for Polygon {
    fn from(p: PolygonT) -> Self {
        let PolygonT { rings, srid } = p;
        Self { rings, srid }
    }
}
impl From<Polygon> for PolygonT {
    fn from(p: Polygon) -> Self {
        let Polygon { rings, srid } = p;
        Self { rings, srid }
    }
}

impl FromSql<Geography, Pg> for Polygon {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        use postgis::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        Ok(Point::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<Geography, Pg> for Polygon {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
        Point::from(*self).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[sql_type = "Geography"]
pub struct Polygon {
    pub rings: Vec<LineStringT<P>>,
    pub srid: Option<i32>,
}

impl From<PolygonT> for Polygon {
    fn from(p: PolygonT) -> Self {
        let PolygonT { rings, srid } = p;
        Self { rings, srid }
    }
}
impl From<Polygon> for PolygonT {
    fn from(p: Polygon) -> Self {
        let Polygon { rings, srid } = p;
        Self { rings, srid }
    }
}

impl FromSql<Geography, Pg> for Polygon {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        use postgis::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        Ok(Point::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<Geography, Pg> for Polygon {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
        Point::from(*self).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[sql_type = "Geography"]
pub struct MultiPolygon {
    pub polygons: Vec<PolygonT>,
    pub srid: Option<i32>,
}

impl From<MultiPolygonT> for MultiPolygon {
    fn from(p: MultiPolygonT) -> Self {
        let MultiPolygonT { polygons, srid } = p;
        Self { polygons, srid }
    }
}
impl From<MultiPolygon> for MultiPolygonT {
    fn from(p: MultiPolygon) -> Self {
        let MultiPolygon { polygons, srid } = p;
        Self { polygons, srid }
    }
}

impl FromSql<Geography, Pg> for MultiPolygon {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        use postgis::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        Ok(Point::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<Geography, Pg> for MultiPolygon {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
        Point::from(*self).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}
