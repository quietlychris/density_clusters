use ndarray::Array;
use rand::Rng;
use std::f32;

use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;
use std::process::Command;

#[derive(Debug, Clone, Copy)]
struct DBSCANPoint {
    x: f32,
    y: f32,
    assigned_cluster: usize,
}

impl DBSCANPoint {
    fn default() -> Self {
        DBSCANPoint {
            x: 0.,
            y: 0.,
            assigned_cluster: 0,
        }
    }

    fn new(
        x: f32,
        y: f32,
        assigned_cluster: usize,
    ) -> Self {
        DBSCANPoint {
            x: x,
            y: y,
            assigned_cluster: assigned_cluster,
        }
    }

    fn calculate_distance(&self, b: DBSCANPoint) -> f32 {
        ((self.x - b.x).powf(2.0) + (self.y - b.y).powf(2.0)).powf(0.5)
    }
}

fn main() {
    //let mut dlist = Array::from_elem((num_points, num_points), 0usize);
    let mut list: Vec<DBSCANPoint> = Vec::new();
    /*
    list.push(DBSCANPoint::new(0.8, 0.8, 0, false, false, 0)); // #0
    list.push(DBSCANPoint::new(1.0, 1.0, 0, false, false, 0)); // #1
    list.push(DBSCANPoint::new(1.2, 3.8, 0, false, false, 0)); // #2
    list.push(DBSCANPoint::new(1.1, 0.7, 0, false, false, 0)); // #3
    list.push(DBSCANPoint::new(1.1, 4.0, 0, false, false, 0)); // #4
    list.push(DBSCANPoint::new(4.0, 3.0, 0, false, false, 0)); // #5
    list.push(DBSCANPoint::new(0.9, 0.9, 0, false, false, 0)); // #6
    list.push(DBSCANPoint::new(1.2, 3.9, 0, false, false, 0)); // #8
    list.push(DBSCANPoint::new(5.0,0.1, 0, false, false, 0)); // #9
    list.push(DBSCANPoint::new(4.9,0.1, 0, false, false, 0)); // #10
    list.push(DBSCANPoint::new(4.9,0.2, 0, false, false, 0)); // #11
    */
    let count = 50;
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let wish: f32 = rng.gen_range(4.,5.);
        let wash: f32 = rng.gen_range(0.,1.);
        list.push(DBSCANPoint::new(wish,wash, 0));
        let pish: f32 = rng.gen_range(0.,1.);
        let posh: f32 = rng.gen_range(0.,1.);
        list.push(DBSCANPoint::new(pish,posh, 0));
        let splish: f32 = rng.gen_range(4.,5.);
        let splosh: f32 = rng.gen_range(4.,5.);
        list.push(DBSCANPoint::new(splish,splosh,0));
        let mish: f32 = rng.gen_range(0.,5.);
        let mosh: f32 = rng.gen_range(0.,5.);
        list.push(DBSCANPoint::new(mish,mosh, 0));
    }

    let epsilon: f32 = 0.9;
    let min_nghbrs = 10;
    let ll = list.len(); // List length
    //println!("list:\n{:#?}",list);
    let mut cn = 0;

    // We'll stay in this loop until there's no more points assigned to cluster #0 (default cluster)
    while list.clone().into_iter().filter(|x| x.assigned_cluster == 0).collect::<Vec<DBSCANPoint>>().len() > 0 {

        for i in 0..ll {
            for j in 0..ll {
                if list[j].assigned_cluster == 0 {
                    //println!("checking if point[{}] belongs the same cluster as point[{}]",j, i);
                    if list[j].calculate_distance(list[i]) < epsilon {
                        list[j].assigned_cluster = list[i].assigned_cluster;
                    }
                }
            }
        }

        //let zeros = Vec::new();
        let index = match list.iter().position(|&x| x.assigned_cluster == 0) {
            Some(n) => n,
            None => break
        };
        list[index].assigned_cluster = cn;
        //println!("index: {:?}",index);
        cn += 1;

        //break;
    }


    //println!("list:\n{:#?}", list);
    make_dbscan_plot(list.clone());

    let mut cluster_counts = Vec::new();
    for _ in 0..cn { cluster_counts.push(0);}
    for point in &list {
        for i in 0..cn {
            if point.assigned_cluster == i {
                cluster_counts[i] +=1;
            }
        }
    }
    //println!("cluster_counts:\n{:?}",cluster_counts);
    let mut total_valid_clusters = 0;
    cluster_counts.iter().for_each(|x| if *x >= min_nghbrs {total_valid_clusters +=1;});
    println!("The total number of valid clusters in this data is: {}",total_valid_clusters);

    //println!("{:.2}", dlist);
}

fn make_dbscan_plot(points: Vec<DBSCANPoint>) {
    //let points = clusters.into_iter().flatten().collect::<Vec<DBSCANPoint>>();
    let mut num_clusters = 0;
    for point in &points {
        if point.assigned_cluster > num_clusters {
            num_clusters = point.assigned_cluster;
        }
    }

    let mut clusters: Vec<Vec<(f64, f64)>> = Vec::with_capacity(num_clusters);
    for _ in 0..(num_clusters + 1) {
        clusters.push(Vec::new());
    }
    for point in &points {
        clusters[point.assigned_cluster].push((point.x as f64, point.y as f64));
    }

    //println!("{:?}", clusters);

    let mut rng = rand::thread_rng();
    let mut scatter_plots: Vec<Plot> = Vec::new();
    for cluster in &clusters {
        let color = format!("#{}", rng.gen_range(0, 999999).to_string(),);
        let s: Plot = Plot::new(cluster.clone()).point_style(
            PointStyle::new()
                .marker(PointMarker::Circle) // setting the marker to be a square
                .colour(&color),
        );
        scatter_plots.push(s);
    }

    let mut v = ContinuousView::new()
        .x_range(0., 5.)
        .y_range(0., 5.)
        .x_label("x-axis")
        .y_label("y-axis");

    for plot in scatter_plots {
        v.representations.push(Box::new(plot.clone()));
    }

    let svg_path = "plot.svg";
    let png_path = "plot.png";
    Page::single(&v).save(&svg_path).unwrap();
    Command::new("cairosvg")
        .arg(svg_path)
        .arg("-o")
        .arg(png_path)
        .output()
        .expect("failed to convert .svg file to .png file");
}
