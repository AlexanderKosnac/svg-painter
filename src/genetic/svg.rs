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
"<g id=\"stroke-0\">
<path fill-opacity=\"1.0\" d=\"m 42.051747,44.929221 c -0.669259,0.260986 0.782116,0.521834 0.04454,0.798105 -0.557793,0.06835 -1.305322,-0.04052 -1.60549,0.401694 -0.52968,-0.480603 -0.696566,0.560062 -1.296628,0.178613 -0.855214,0.238187 -1.666909,-0.05097 -2.511678,-0.03397 -0.860394,0.163376 -1.562174,-0.391781 -2.372257,-0.517633 -0.687845,-0.07012 -0.315775,-0.779652 -0.978056,-0.775045 0.183438,-0.255491 -0.26988,-0.405863 0.06947,-0.793197 0.0066,-0.673014 0.898029,-0.297149 1.165466,-0.808711 0.882575,-0.13121 1.768835,-0.207762 2.655366,-0.288278 0.679815,0.177757 1.335866,0.208357 2.04508,0.192323 0.931889,0.168993 1.838879,0.534708 2.620835,1.06438 0.397518,0.332974 -0.255733,0.12701 0.09463,0.13074 0.06888,0.137117 0.09603,0.299148 0.06834,0.45089\"/>
<path fill-opacity=\"0.7\" d=\"m 42.994876,45.210205 c -0.776866,-0.156923 -0.696877,0.916376 -1.524295,0.935132 -1.138592,0.269257 -2.324081,0.23303 -3.486262,0.257812 -0.687512,-0.154329 -1.35173,-0.311146 -2.044168,-0.402922 -0.566725,-0.464133 -1.651666,-0.282088 -1.708423,-1.162853 0.214067,-0.241663 0.688094,-0.321155 0.19181,-0.596669 0.304074,-0.130817 0.511847,-0.374933 0.480831,-0.444142 0.784255,-0.164113 1.471774,-0.697232 2.315149,-0.523595 0.930018,-0.05271 1.901665,-0.163153 2.793102,0.17952 0.848385,-0.01907 1.708681,0.255502 2.385555,0.757953 0.286267,0.271132 0.460567,0.635333 0.596485,0.999582\"/>
<path fill-opacity=\"0.4\" d=\"m 43.72608,45.078621 c -0.212985,0.289705 -0.610834,0.372819 -0.660548,0.750214 -0.269404,0.347618 -0.929967,0.115277 -1.104082,0.267832 0.388876,0.294801 -0.636597,0.235838 -0.835997,0.153141 -0.573426,-0.05188 -1.021336,0.40974 -1.635607,0.153003 -0.705783,-0.01236 -1.391554,-0.190108 -2.093271,-0.245155 -0.52798,0.114074 -0.796854,-0.466839 -1.242964,-0.522156 -0.564199,-0.06636 -1.056033,-0.979832 -0.553231,-1.39868 0.572397,0.08366 0.16831,-0.528252 0.691742,-0.484706 0.428842,-0.339044 0.959446,-0.228807 1.460915,-0.262033 0.721145,-0.07957 1.443902,-0.230116 2.170465,-0.161643 0.57792,-0.09696 1.08767,0.258074 1.648442,0.254245 0.563395,0.291433 1.295276,0.369682 1.725512,0.844197 -0.472248,0.361338 0.653526,0.126443 0.428496,0.651623\"/>
</g>",
"<g id=\"stroke-1\">
<path fill-opacity=\"1.0\" d=\"m 38.258802,51.965108 c -0.21963,0.300625 0.638101,0.738864 -0.06581,0.862436 -0.366029,0.01073 -0.396712,0.49391 -0.689187,0.421001 -0.121977,0.489151 -0.658915,0.427615 -1.072058,0.469608 -0.40301,-0.09461 -0.861339,0.08611 -1.206185,-0.220186 -0.31901,-0.237091 -0.709782,-0.440623 -0.689585,-0.878799 -0.251105,-0.437597 -0.104888,-0.974701 0.134755,-1.38055 0.312079,-0.197007 0.471456,-0.586008 0.911033,-0.588949 0.334111,-0.195354 0.705519,-0.191494 1.057317,-0.0166 0.373218,-0.232779 0.787354,0.176727 1.082563,0.39613 0.232381,0.139529 0.509631,0.554219 0.46938,0.567827 0.07442,0.109357 0.02967,0.248208 0.0675,0.367936\"/>
<path fill-opacity=\"0.7\" d=\"m 38.710654,52.141299 c -0.243451,-0.03846 -0.408363,0.314902 -0.157804,0.414483 -0.01514,0.196076 -0.329582,0.292122 -0.180813,0.513022 -0.193783,0.16065 -0.426209,0.297669 -0.648941,0.427824 -0.204561,0.07889 -0.430619,0.07769 -0.637679,0.155183 -0.192019,0.03986 -0.421298,0.13785 -0.603156,0.0528 -0.25953,0.08496 -0.385433,-0.277412 -0.666587,-0.23216 -0.261616,0.114106 -0.468448,-0.19924 -0.521798,-0.400252 -0.09144,-0.15878 -0.629369,-0.118089 -0.40128,-0.373519 -0.116982,-0.22085 -0.14088,-0.583861 0.148771,-0.672176 0.05597,-0.148512 -0.288601,-0.384116 0.02382,-0.510422 0.209476,0.005 -0.01382,-0.402211 0.264799,-0.334039 0.127678,-0.167261 0.269798,-0.239611 0.332429,-0.420199 0.211751,-0.128448 0.476941,-0.04798 0.699366,-0.165175 0.240343,-0.06679 0.503179,-0.148686 0.734396,-0.0048 0.168132,0.122336 0.442265,0.06433 0.562345,0.06655 0.08299,0.271847 0.46307,0.239083 0.575099,0.468213 0.0945,0.225935 0.283834,0.406693 0.336089,0.654723 -0.003,0.142526 0.112438,0.229503 0.140923,0.359893\"/>
<path fill-opacity=\"0.4\" d=\"m 38.999437,51.926476 c -0.185018,0.296501 -0.02312,0.70765 -0.26637,0.975349 -0.209085,0.102954 -0.320399,0.310587 -0.493635,0.448134 -0.231767,0.02374 -0.429963,0.141635 -0.637199,0.248602 -0.19503,0.10435 -0.422048,0.1044 -0.629737,0.04148 -0.311405,-0.05579 -0.630477,-0.07087 -0.935947,-0.157779 -0.178233,-0.319409 -0.613176,-0.480561 -0.637836,-0.884058 -0.07446,-0.293506 -0.123561,-0.627242 0.05591,-0.895059 0.147992,-0.201251 0.148168,-0.480334 0.347905,-0.64815 0.148523,-0.152881 0.361208,-0.215465 0.566574,-0.231565 0.360677,-0.129399 0.736045,-0.382827 1.132674,-0.238163 0.412717,0.119316 0.785226,0.34722 1.120601,0.61142 -0.0763,0.324208 0.400904,0.422498 0.376911,0.729618\"/>
</g>",
"<g id=\"stroke-2\">
<path fill-opacity=\"1.0\" d=\"m 34.269382,54.000857 c 0.267222,-0.09023 0.905175,-0.01832 1.555001,-0.143634 0.738392,-0.131521 1.464113,0.03905 2.193227,0.125268 0.731396,0.05104 1.44418,-0.148879 2.167109,-0.178851 0.637772,0.259942 1.327732,0.21533 2.002281,0.193768 -0.618984,0.533826 -0.146769,1.40714 -0.277668,2.095346 -0.06761,1.036403 0.119161,2.060726 0.159733,3.094542 0.172524,0.825144 -0.302315,1.624113 -0.05291,2.48142 -0.08901,0.849814 -1.274903,0.104228 -1.845526,0.267464 -0.900061,-0.03765 -1.79505,0.151055 -2.687854,0.0311 -0.7772,-0.0948 -1.277868,0.221985 -2.041135,-0.07675 -0.590484,-0.167778 -1.723856,0.534468 -1.401044,-0.416256 -0.205045,-0.847404 0.0017,-2.009081 -0.01854,-2.875147 0.0254,-0.73151 -0.11198,-1.59511 0.0328,-2.330451 -0.108588,-0.800565 -0.132032,-1.622308 -0.05506,-2.328128\"/>
<path fill-opacity=\"0.7\" d=\"m 42.1303,54.031458 c 0.337277,-0.208583 0.398498,0.394835 0.258382,0.815734 0.01561,0.572346 0.02256,1.145161 -0.04813,1.715081 0.0092,0.565861 0.07241,1.132271 0.06069,1.698966 0.06992,0.664962 -0.265851,1.333261 -0.03435,1.988073 0.119825,0.57424 0.13363,1.17908 0.04642,1.726366 -0.202937,0.167264 -1.027859,0.169074 -0.785987,-0.1777 -0.01932,-0.610392 0.165215,-1.263976 0.242085,-1.873125 -0.01262,-0.637404 0.06412,-1.274467 -0.02237,-1.908827 -0.05047,-0.445853 0.0095,-0.765939 -0.08131,-1.214214 -0.06411,-0.469447 0.06173,-1.033982 0.113126,-1.508432 0.007,-0.46886 -0.621385,-0.722138 -0.22264,-0.968734 z\"/>
<path fill-opacity=\"0.4\" d=\"m 41.772298,54.014303 c 0.287357,0.0575 1.021931,-0.215166 1.035663,0.269696 0.07822,0.899191 0.240674,1.776759 0.04936,2.683758 -0.136296,0.44861 -0.09249,0.922534 -0.06879,1.382226 -0.03206,0.44879 -0.0756,0.897802 -0.051,1.348005 -0.005,0.685293 0.0042,1.804316 -0.129661,2.113333 -0.268655,0.285926 -0.496445,0.288797 -0.745243,0.264149 -0.489296,0.03069 -0.01644,-0.65265 -0.08443,-0.931708 -0.03689,-0.474575 -0.108489,-0.9542 -0.01558,-1.427245 0.03054,-0.499283 -0.05467,-0.99786 -0.01636,-1.497625 0.004,-0.561455 -0.0048,-1.12417 0.06618,-1.682219 0.01562,-0.234356 0.0076,-0.475299 -0.07055,-0.699023 0.04103,-0.606516 0.0758,-1.216102 0.0304,-1.823347 z\"/>
</g>"
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
