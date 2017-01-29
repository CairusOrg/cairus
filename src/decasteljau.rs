use std::ops::Shr;

struct Point{
    x: f32,
    y: f32,
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

    fn copy(p: Point)->Point{

        Point{
            x: p.x,
            y: p.y,
        }
    }

    fn lerp_half(a:Point, b:Point, mut result:Point)->Point{

        result = Point::create(a.x + (b.x - a.x), a.y + (b.y - a.y));

        return result;

    }
}

struct SplineKnots{

    a: Point,
    b: Point,
    c: Point,
    d: Point,

}

impl SplineKnots{

    fn create(a:Point, b:Point, c:Point, d:Point)->SplineKnots{

        SplineKnots{
            a:a,
            b:b,
            c:c,
            d:d,
        }


    }




}


struct DeCasteljau{

    pub ab: Point,
    pub bc: Point,
    pub cd: Point,
    abbc: Point,
    bccd: Point,
    fin: Point,



}


impl DeCasteljau {

    fn create_spline(&self, s1: SplineKnots, s2: SplineKnots)->DeCasteljau{

        DeCasteljau{
            ab: Point::create(s1.a.x + s1.b.x - s1.a.x, s1.a.y + s1.b.y-s1.a.y),
            bc: Point::create(s1.b.x + s1.c.x - s1.b.x, s1.b.y + s1.c.y - s1.b.y),
            cd: Point::create(s1.c.x + s1.d.x - s1.c.x, s1.c.y + s1.d.y - s1.c.y),
            abbc: Point::create(self.ab.x + self.bc.x - self.ab.x, self.ab.y + self.bc.y - self.ab.y),
            bccd: Point::create(self.bc.x + self.cd.x - self.bc.x, self.bc.y + self.cd.y - self.cd.y),
            fin: Point::create(self.abbc.x + self.bccd.x - self.abbc.x, self.abbc.y + self.bccd.y - self.abbc.y),

        }


    }



}

mod tests{

    use::decasteljau::Point;
    use::decasteljau::SplineKnots;
    use::decasteljau::DeCasteljau;

    #[test]
    fn test_create_splineknots(){

        let p1 = Point::create(0.,0.);
        let p2 = Point::create(1., 2.);
        let p3 = Point::create(1.5, 2.4);
        let p4 = Point::create(2.6, 3.3);



    }

    #[test]
    fn test_create_decasteljau(){

        let p1 = Point::create(0.,0.);
        let p2 = Point::create(1., 2.);
        let p3 = Point::create(1.5, 2.4);
        let p4 = Point::create(2.6, 3.3);

        let p5 = Point::create(0.,0.);
        let p6 = Point::create(1., 6.);
        let p7 = Point::create(1.5, 9.4);
        let p8 = Point::create(2.6, 3.3);



        let d1 = DeCasteljau{
            ab: p1,
            bc: p2,
            cd: p3,
            abbc: p4,
            bccd: Point::origin(),
            fin: Point::origin(),
            //s4: s2,
        };



        //let d2 = DeCasteljau::create_spline(d1, s1, s2);


    }

}



