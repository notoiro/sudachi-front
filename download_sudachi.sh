#!/bin/bash

git clone "https://github.com/WorksApplications/sudachi.rs.git"
cd ./sudachi.rs

./fetch_dictionary.sh latest full
