## about 

- study repo for https://github.com/mpizenberg/visual-odometry-rs

## how to run 

- download a dataset, eg `fr1/xyz` from https://cvg.cit.tum.de/data/datasets/rgbd-dataset/download
- use the `associate.py` in scripts folder to generate associations.txt
  - `python associate.py depth.txt rgb.txt > associations.txt`
- run the repo `cargo run --release --bin vors_track -- fr1 /path/to/some/freiburg1/dataset/associations.txt`
