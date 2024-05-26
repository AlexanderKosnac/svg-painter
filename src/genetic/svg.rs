use std::cmp;
use rand::Rng;

use crate::genetic::color::Rgba;

use crate::genetic::{Base, Genome};

pub struct RgbBase {
    color: Rgba,
}

impl Base for RgbBase {
    fn new(max_x: u32, max_y: u32) -> Self {
        Self {
            color: Rgba::new_rand(),
        }
    }

    fn express(&self) -> String {
        return self.color.as_hex();
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        self.color.mutate(rng.gen_range(0.0..20.0));
    }
}

impl Clone for RgbBase {
    fn clone(&self) -> Self {
        Self {
            color: self.color.clone(),
        }
    }
}

pub struct CircleBase {
    x: i32,
    y: i32,
    r: i32,
    color: Rgba,
    max_r: u32,
}

impl Base for CircleBase {
    fn new(max_x: u32, max_y: u32) -> Self {
        let mut rng = rand::thread_rng();
        let max_r = (max_x+max_y)/2/16;
        Self {
            x: rng.gen_range(0..max_x) as i32,
            y: rng.gen_range(0..max_y) as i32,
            r: rng.gen_range(1..max_r) as i32,
            color: Rgba::new_rand(),
            max_r: max_r,
        }
    }

    fn express(&self) -> String {
        return format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill-opacity=\"{:.3}\" fill=\"{}\"/>", self.x, self.y, self.r, (self.color.a as f64)/255.0, self.color.as_hex());
    }

    fn mutate(&mut self) {
        let m = 5;
        let mut rng = rand::thread_rng();
        self.x = self.x + rng.gen_range(-m..m);
        self.y = self.y + rng.gen_range(-m..m);
        self.r = cmp::max(self.r + rng.gen_range(-m..m), 1);
        self.color.mutate(rng.gen_range(0.0..20.0));
    }
}

impl Clone for CircleBase {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            r: self.r,
            color: self.color.clone(),
            max_r: self.max_r,
        }
    }
}

pub struct TriangleBase {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    color: Rgba,
}

impl Base for TriangleBase {
    fn new(max_x: u32, max_y: u32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x1: rng.gen_range(0..max_x) as i32,
            y1: rng.gen_range(0..max_y) as i32,
            x2: rng.gen_range(0..max_x) as i32,
            y2: rng.gen_range(0..max_y) as i32,
            x3: rng.gen_range(0..max_x) as i32,
            y3: rng.gen_range(0..max_y) as i32,
            color: Rgba::new_rand(),
        }
    }

    fn express(&self) -> String {
        let point_seq = format!("{},{} {},{} {},{}", self.x1, self.y1, self.x2, self.y2, self.x3, self.y3);
        return format!("<polygon points=\"{}\" fill-opacity=\"{:.3}\" fill=\"{}\"/>", point_seq, (self.color.a as f64)/255.0, self.color.as_hex());
        
    }

    fn mutate(&mut self) {
        let m = 5;
        let mut rng = rand::thread_rng();
        self.x1 = self.x1 + rng.gen_range(-m..m);
        self.y1 = self.y1 + rng.gen_range(-m..m);
        self.x2 = self.x2 + rng.gen_range(-m..m);
        self.y2 = self.y2 + rng.gen_range(-m..m);
        self.x3 = self.x3 + rng.gen_range(-m..m);
        self.y3 = self.y3 + rng.gen_range(-m..m);
        self.color.mutate(rng.gen_range(0.0..20.0));
    }
}

impl Clone for TriangleBase {
    fn clone(&self) -> Self {
        Self {
            x1: self.x1,
            y1: self.y1,
            x2: self.x2,
            y2: self.y2,
            x3: self.x3,
            y3: self.y3,
            color: self.color.clone(),
        }
    }
}

static STROKES: [&str; 3] = [
    "<path id=\"stroke-0\" d=\"m 14.091296,1.3528368 c -0.515176,0.1980335 -0.0047,0.3374416 0.30714,0.4537773 0.581845,0.1143003 -0.0075,0.4054002 -0.951653,0.4697578 -0.361903,0.00889 -0.627164,0.052324 -0.842414,0.1114836 -0.3855,0.105946 -0.603662,0.2618238 -0.896682,0.3367719 C 10.944988,2.6895585 10.523646,2.8681956 10.633766,3.0145485 10.112304,3.1798938 9.1552812,3.1805759 8.4146106,3.2399059 7.0286919,3.2760454 5.6644573,3.1124355 4.2804581,3.076467 2.9571027,2.8030879 1.4296835,2.5091313 0.7926589,2.0804891 0.72140268,1.8898934 0.84534296,1.7160337 0.09298035,1.5079284 -0.20296711,1.3021189 0.32727123,1.2027164 0.04951586,0.99245285 -0.00522718,0.71496635 0.25901359,0.46990445 1.0265317,0.35047365 c 0.8010408,0.0136071 0.6130959,-0.2872777 1.4915809,-0.2687996 0.6057293,-0.0238879 1.1580506,-0.0689378 1.853642,-0.0474113 0.9553645,-0.0105871 2.0280454,-0.0878214 3.0171994,0.0318599 0.3234554,0.066239 0.7492746,0.2323044 1.0584005,0.067706 0.7781051,-0.1192194 1.7219075,-0.014201 2.3473855,0.078583 0.598496,0.095591 1.156835,0.1952899 1.68032,0.2994107 0.423505,0.08423 0.822293,0.1709921 1.198487,0.2601339 0.265327,0.049537 0.526044,0.207813 0.08091,0.28776035 -0.195718,-0.070703 0.159039,-0.17856475 0.200779,-0.052303 0.164893,0.1004342 0.169578,0.2226013 0.135314,0.34538\"/>",
    "<path id=\"stroke-1\" d=\"m 13.071983,1.8789036 c -0.678327,0.05184 -0.967649,0.243326 -1.016714,0.425699 0,0 0,0 0,0 0,0 0,0 0,0 0,0 0,0 0,0 0,0 0,0 0,0 0,0 0,0 0,0 0,0 0,0 0,0 -0.0054,0.02006 -0.0079,0.04007 -0.0077,0.05991 0.002,0.001 0.0039,0.0021 0.0059,0.0031 0,0 0,0 0,0 0.201162,0.107031 0.234334,0.248328 -0.0095,0.326542 -0.122649,0.0393 -0.317032,0.06381 -0.602473,0.06368 -0.634184,0.02156 -0.616134,0.216479 -0.495635,0.296306 -0.22017,-0.06394 -0.56463,-0.09052 -0.82669,-0.08002 -0.2806005,0.01114 -0.4638403,0.06686 -0.2760367,0.179179 0.1820107,0.189759 -0.9046201,0.0669 -1.3243943,0.06218 C 7.8488384,3.1731036 7.1880502,3.1045256 6.5309273,3.0858906 6.0276117,3.2282416 4.979901,3.1106926 4.7306599,2.9214046 4.5400536,2.8391226 4.2691408,2.7822876 3.9644687,2.7445756 3.6525627,2.7056736 3.3098393,2.6874716 2.9702199,2.6766056 2.4901193,2.6568586 1.9867263,2.6517586 1.5201357,2.6230116 1.3483748,2.6124646 1.1818593,2.5986756 1.0237106,2.5798356 0.73929414,2.4700706 0.6412101,2.3163736 0.69690941,2.1987326 c -0.72145085,-0.0585 -0.9201638,-0.245022 -0.4190729,-0.425833 0.52368981,-0.194662 0.14756554,-0.366028 0.0976887,-0.552437 -0.042161,-0.144012 -0.50469164,-0.39124498 0.42240938,-0.45679598 0.60083341,-0.03842 0.49172081,-0.205852 0.64782301,-0.327534 0.07217,-0.05627 0.202739,-0.1022238 0.4912836,-0.1154618 0.676942,-0.01883 0.9232969,-0.2492442 1.728063,-0.2163204 0.1068563,-0.00665 0.217986,-0.00724 0.3308606,-0.00445 0.4619163,0.011881 0.9465288,0.081217 1.2722062,0.0061 0.5516391,-0.091454 1.2834762,-0.1588794 1.9999543,-0.049352 0.3821779,0.058965 0.6959751,0.2810408 1.0423713,0.1384684 0.7478008,-0.011566 1.3035241,0.1872275 1.8258064,0.2982885 1.02e-4,2.6e-5 0.02936,0.0077 0.02946,0.0077 0.659069,0.173015 1.251875,0.352856 1.926443,0.53784098 0,0 0,0 0,0 0.136252,0.0374 0.274158,0.07477 0.41754,0.112604 0.366206,0.117707 0.955516,0.387803 0.375459,0.438231 -0.403294,-0.06124 -0.0055,-0.30456 0.110587,-0.108824 0.07559,0.134488 0.09303,0.267715 0.07556,0.397825\"/>",
    "<path id=\"stroke-2\" d=\"M 0.01718467,1.6896946 C 0.78284207,1.5498046 1.0556384,1.3170966 1.0413355,1.1173836 0.71652977,0.98113858 0.74232967,0.72792558 1.6629944,0.69422758 c 0.6592834,-0.04567 0.6445792,-0.233259 0.5114251,-0.312607 0.2327267,0.05785 0.5942447,0.07605 0.8587353,0.05739 0.2804094,-0.0198 0.4575018,-0.08195 0.2665263,-0.187024 -0.1851054,-0.17935 0.9050101,-0.09973 1.3307381,-0.116213 0.67417,0.0087 1.3411596,0.03393 1.9982451,-0.0031 0,0 8.9e-5,-3.3e-5 8.9e-5,-3.3e-5 0,0 0,0 0,0 0.481931,-0.180923 1.571844,-0.171889 1.855279,-0.0026 0.196284,0.06139 0.465878,0.09389 0.772129,0.110126 0.345408,0.01801 0.725167,0.01645 1.0804057,0.0036 0.159145,-0.0043 0.317808,-0.01106 0.475218,-0.01987 0,0 0,0 0,0 0.494256,-0.02709 0.987565,-0.07597 1.424941,-0.09256 0.290576,0.06418 0.415563,0.195107 0.381498,0.321707 0.683123,-0.06563 0.986344,0.04435 0.493902,0.339369 -0.488452,0.29893202 -0.0818,0.38323802 0.0075,0.56185302 0.07053,0.132915 0.607934,0.296948 -0.358942,0.530906 -0.919081,0.216287 -0.186,0.461398 -1.157593,0.611727 -0.211111,0.02908 -0.378507,0.06712 -0.536335,0.109862 -0.379564,0.102376 -0.695395,0.23602 -1.2735187,0.249864 -0.124093,0.01568 -0.252852,0.02376 -0.382074,0.02858 -0.441236,0.01689 -0.909889,-0.0029 -1.222102,0.09853 -0.401409,0.103635 -0.915439,0.207986 -1.46804,0.233553 -0.2000406,0.0091 -0.4029414,0.0078 -0.6049455,-0.0069 -0.4087963,-0.03034 -0.7504988,-0.231783 -1.1066101,-0.06972 -0.783266,0.0538 -1.3833332,-0.125542 -1.9291341,-0.220934 -5.48e-5,-1.2e-5 -0.024165,-0.0053 -0.024219,-0.0053 0,0 -1e-7,0 -1e-7,0 -0.8378157,-0.185516 -1.5323664,-0.397824 -2.40167873,-0.554063 -0.3638213,-0.08553 -1.0037189,-0.293861 -0.4164719,-0.402686 0.4095553,0.02051 0.038327,0.304165 -0.1005825,0.119534 -0.090231,-0.125951 -0.1230824,-0.256051 -0.1195388,-0.38747\"/>",
];

pub struct StrokeBase {
    stroke_idx: usize,
    x: i32,
    y: i32,
    rotation: i32,
    scale_x: f32,
    scale_y: f32,
    color: Rgba,
}

impl Base for StrokeBase {
    fn new(max_x: u32, max_y: u32) -> Self {
        let mut rng = rand::thread_rng();
        let scale = rng.gen_range(0.5..2.0);
        Self {
            stroke_idx: rng.gen_range(0..STROKES.len()) as usize,
            x: rng.gen_range(0..max_x) as i32,
            y: rng.gen_range(0..max_y) as i32,
            rotation: rng.gen_range(0..360) as i32,
            scale_x: scale,
            scale_y: scale,
            color: Rgba::new_rand(),
        }
    }

    fn express(&self) -> String {
        let stroke = format!("<use href=\"#stroke-{}\"/>", self.stroke_idx);
        let transformations = format!("translate({} {}) rotate({}) scale({:.5} {:.5})", self.x, self.y, self.rotation, self.scale_x, self.scale_y);
        return format!("<g fill-opacity=\"{:.3}\" fill=\"{}\" transform=\"{}\">{}</g>", (self.color.a as f64)/255.0, self.color.as_hex(), transformations, stroke);
    }

    fn mutate(&mut self) {
        let m = 5;
        let mut rng = rand::thread_rng();
        //self.stroke_idx = (self.stroke_idx + rng.gen_range(0..1)) % 3;
        self.x = self.x + rng.gen_range(-m..m);
        self.y = self.y + rng.gen_range(-m..m);
        self.rotation = (self.rotation + rng.gen_range(-m..m)) % 360;
        self.scale_x = self.scale_x + rng.gen_range(-0.3..0.3);
        self.scale_y = self.scale_y + rng.gen_range(-0.3..0.3);
        self.color.mutate(rng.gen_range(0.0..20.0));
    }
}

impl Clone for StrokeBase {
    fn clone(&self) -> Self {
        Self {
            stroke_idx: self.stroke_idx,
            x: self.x,
            y: self.y,
            rotation: self.rotation,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            color: self.color.clone(),
        }
    }
}

pub struct SvgElementGenome<T: Base> {
    sequence: Vec<T>,
    bg_base: RgbBase,
    width: u32,
    height: u32,
}

impl<T: Base> Genome for SvgElementGenome<T> {
    fn new(genome_size: u32, width: u32, height: u32) -> Self {
        Self {
            sequence: (0..genome_size).map(|_| T::new(width, height)).collect(),
            bg_base: RgbBase::new(width, height),
            width: width,
            height: height,
        }
    }

    fn express(&self) -> String {
        let expressed: Vec<String> = self.sequence.iter().map(|b| b.express()).collect();
        let expressed_string: String = expressed.join("\n");
        return format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n<def>\n{}\n</def>\n<rect width=\"100%\" height=\"100%\" fill=\"{}\"/>\n{expressed_string}\n</svg>", self.width, self.height, STROKES.join("\n"), self.bg_base.express());
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        if 0.9 > rng.gen_range(0.0..1.0) {
            self.bg_base.mutate();
        }

        let idx = rng.gen_range(0..self.sequence.len()) as usize;
        let mut candidate = &mut self.sequence[idx];
        candidate.mutate();
    }

    fn insertion(&mut self) {
        self.sequence.push(T::new(self.width, self.height));
    }

    fn len(&self) -> usize {
        return self.sequence.len();
    }
}

impl<T: Base + Clone> Clone for SvgElementGenome<T> {
    fn clone(&self) -> Self {
        Self {
            sequence: self.sequence.clone(),
            bg_base: self.bg_base.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

unsafe impl<T: Base> Send for SvgElementGenome<T> {}
