# snakemeter

Statistical [Python](https://www.python.org/) Profiler written in [Rust](http://www.rust-lang.org/)


# Command-line Usage

  1) ./build_release.sh

  2) python snakemeter.py \<your_python_script\>
  
# Program Usage

```python

import snakemeter
sampler = snakemeter.Sampler(rate=500)
sampler.start()

try:
  # your code
finally:
  sampler.stop()

  sampler.print_stats()

```


# Limitations

Currently supports only CPython 2.7.x


