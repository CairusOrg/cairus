use std::f32;


//Creates points for spline
pub struct Point{
    pub x: f32,
    pub y: f32,
}

impl Point{


    fn origin()->Point{
        Point{
            x:0.,
            y:0.,
        }
    }

    fn create(x:f32, y:f32)->Point{

        Point{
            x: x,
            y: y,
        }

    }

//lerp_half as coded in cairo c. However not sure how to implement bit shift >>1
    fn lerp_half(a:Point, b:Point, mut result:Point)->Point{

        result = Point::create(a.x + (b.x - a.x), a.y + (b.y - a.y));

        return result;

    }
}

//SplineKnots as in cairo c
pub struct SplineKnots{

    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,

}

impl SplineKnots{

    pub fn create(a:Point, b:Point, c:Point, d:Point)->SplineKnots{

        SplineKnots{
            a:Point::create(a.x, a.y),
            b:Point::create(b.x, b.y),
            c:Point::create(c.x, c.y),
            d:Point::create(d.x, d.y),
        }


    }


}

//separated points and knots. This is the implementation of points
struct DeCasteljau_Points{

    pub ab: Point,
    pub bc: Point,
    pub cd: Point,
    abbc: Point,
    bccd: Point,
    fin: Point,

}


impl DeCasteljau_Points {

    fn create()->DeCasteljau_Points{

        DeCasteljau_Points{
            ab: Point::origin(),
            bc: Point::origin(),
            cd: Point::origin(),
            abbc: Point::origin(),
            bccd: Point::origin(),
            fin: Point::origin(),


        }

    }


    fn create_spline(&self, s1: SplineKnots, s2: SplineKnots)->DeCasteljau_Points{

        DeCasteljau_Points{
            ab: Point::create(s1.a.x + s1.b.x - s1.a.x, s1.a.y + s1.b.y-s1.a.y),
            bc: Point::create(s1.b.x + s1.c.x - s1.b.x, s1.b.y + s1.c.y - s1.b.y),
            cd: Point::create(s1.c.x + s1.d.x - s1.c.x, s1.c.y + s1.d.y - s1.c.y),
            abbc: Point::create(self.ab.x + self.bc.x - self.ab.x, self.ab.y + self.bc.y - self.ab.y),
            bccd: Point::create(self.bc.x + self.cd.x - self.bc.x, self.bc.y + self.cd.y - self.cd.y),
            fin: Point::create(self.abbc.x + self.bccd.x - self.abbc.x, self.abbc.y + self.bccd.y - self.abbc.y),


        }


    }


}

//implementation of knots. Had some syntax issue when defing s1 with same variables as s2
struct knots{

    s2:SplineKnots,
}

impl knots{

    fn create(d1: DeCasteljau_Points, s1: SplineKnots)->knots{


        knots{

            s2:SplineKnots::create(d1.fin, d1.bc, d1.cd, s1.d),

        }
    }
}

impl PartialEq for SplineKnots {
    fn eq(&self, other: &SplineKnots) -> bool {
        self.a.x == other.a.x && self.b.x == other.b.x &&
            self.c.x == other.c.x && self.d.x == other.d.x
    }
}



mod tests{

    use::decasteljau::Point;
    use::decasteljau::SplineKnots;
    use::decasteljau::DeCasteljau_Points;
    use::decasteljau::knots;

    #[test]
    fn test_create_splineknots(){

        let p1 = Point::create(0.,0.);
        let p2 = Point::create(1., 2.);
        let p3 = Point::create(1.5, 2.4);
        let p4 = Point::create(2.6, 3.3);

        let p5 = Point::create(0.,0.);
        let p6 = Point::create(1., 2.);
        let p7 = Point::create(1.5, 2.4);
        let p8 = Point::create(2.6, 3.3);

        let s1 = SplineKnots::create(p1, p2, p3, p4);
        let s2 = SplineKnots::create(p5, p6, p7, p8);

        let d1 = DeCasteljau_Points::create();
        let d2 = &d1;
        let d2 = DeCasteljau_Points::create_spline(d2, s1, s2);






    }

    #[test]
    fn test_create_decasteljau(){

        let p5 = Point::create(0.,0.);
        let p6 = Point::create(1., 2.);
        let p7 = Point::create(1.5, 2.4);
        let p8 = Point::create(2.6, 3.3);













        //let d2 = DeCasteljau::create_spline(d1, s1, s2);


    }

}



