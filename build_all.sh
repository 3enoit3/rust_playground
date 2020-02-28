for f in *.rs
do
    echo "Building $f .."
    rustc $f
done

for f in */Cargo.toml
do
    d=$(dirname $f)
    cd $d
    echo "Building $d .."
    cargo build
    echo "Testing $d .."
    cargo test
    cd ..
done
