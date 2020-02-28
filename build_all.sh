for f in *.rs
do
    echo "Building $f .."
    rustc $f
done
