
<!-- README.md is generated from README.Rmd. Please edit that file -->

# rparquet

<!-- badges: start -->
<!-- badges: end -->

The goal of rparquet is to provide a nanoarrow-based Parquet file reader
with minimal install footprint. It’s currently an experiment and barely
works at all (see Example).

## Installation

You can install the development version of rparquet from
[GitHub](https://github.com/) with:

``` r
# install.packages("remotes")
remotes::install_github("paleolimbot/rparquet")
```

Installation requires a rust compiler.

## Example

This is a basic example which shows you how to solve a common problem:

``` r
library(rparquet)

schema <- nanoarrow::nanoarrow_allocate_schema()

rparquet:::parquet_read_schema(
  "inst/extdata/penguins.parquet",
  nanoarrow::nanoarrow_pointer_addr_chr(schema)
)
#> NULL

schema
#> <nanoarrow_schema struct>
#>  $ format    : chr "+s"
#>  $ name      : chr ""
#>  $ metadata  : list()
#>  $ flags     : int 0
#>  $ children  :List of 17
#>   ..$ studyName          :<nanoarrow_schema string>
#>   .. ..$ format    : chr "u"
#>   .. ..$ name      : chr "studyName"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Sample Number      :<nanoarrow_schema double>
#>   .. ..$ format    : chr "g"
#>   .. ..$ name      : chr "Sample Number"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Species            :<nanoarrow_schema string>
#>   .. ..$ format    : chr "u"
#>   .. ..$ name      : chr "Species"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Region             :<nanoarrow_schema string>
#>   .. ..$ format    : chr "u"
#>   .. ..$ name      : chr "Region"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Island             :<nanoarrow_schema string>
#>   .. ..$ format    : chr "u"
#>   .. ..$ name      : chr "Island"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Stage              :<nanoarrow_schema string>
#>   .. ..$ format    : chr "u"
#>   .. ..$ name      : chr "Stage"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Individual ID      :<nanoarrow_schema string>
#>   .. ..$ format    : chr "u"
#>   .. ..$ name      : chr "Individual ID"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Clutch Completion  :<nanoarrow_schema string>
#>   .. ..$ format    : chr "u"
#>   .. ..$ name      : chr "Clutch Completion"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Date Egg           :<nanoarrow_schema date32>
#>   .. ..$ format    : chr "tdD"
#>   .. ..$ name      : chr "Date Egg"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Culmen Length (mm) :<nanoarrow_schema double>
#>   .. ..$ format    : chr "g"
#>   .. ..$ name      : chr "Culmen Length (mm)"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Culmen Depth (mm)  :<nanoarrow_schema double>
#>   .. ..$ format    : chr "g"
#>   .. ..$ name      : chr "Culmen Depth (mm)"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Flipper Length (mm):<nanoarrow_schema double>
#>   .. ..$ format    : chr "g"
#>   .. ..$ name      : chr "Flipper Length (mm)"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Body Mass (g)      :<nanoarrow_schema double>
#>   .. ..$ format    : chr "g"
#>   .. ..$ name      : chr "Body Mass (g)"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Sex                :<nanoarrow_schema string>
#>   .. ..$ format    : chr "u"
#>   .. ..$ name      : chr "Sex"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Delta 15 N (o/oo)  :<nanoarrow_schema double>
#>   .. ..$ format    : chr "g"
#>   .. ..$ name      : chr "Delta 15 N (o/oo)"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Delta 13 C (o/oo)  :<nanoarrow_schema double>
#>   .. ..$ format    : chr "g"
#>   .. ..$ name      : chr "Delta 13 C (o/oo)"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>   ..$ Comments           :<nanoarrow_schema string>
#>   .. ..$ format    : chr "u"
#>   .. ..$ name      : chr "Comments"
#>   .. ..$ metadata  : list()
#>   .. ..$ flags     : int 2
#>   .. ..$ children  : NULL
#>   .. ..$ dictionary: NULL
#>  $ dictionary: NULL
```
