### Density-based clustering

This a pure Rust implementation of a naive density-based clustering algorithm similar to DBSCAN.

Here, 50 points are located in 1x1 boxes in each of the corners of quadrants I,III, and IV, with an additional 50 points spread across the entire `(0,0),(5,5)` range, for a total of two hundreds points. After some tuning of the minimal distance and minimal required number of points per cluster, the algorithms properly identifies three clusters with several outliers.

<p align="center"><img src="/example_plot.png" width="600" height="500" /></p>
