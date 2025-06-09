cargo build --release

mkdir temp_extract
cd temp_extract
powerpc-eabi-ar x ../target/powerpc-unknown-eabi/release/libcustom_functions.a

cd ..
powerpc-eabi-ld -r -T merge.ld -o custom_functions.o temp_extract/*.o

powerpc-eabi-readelf -S custom_functions.o | wc -l

rm -rf temp_extract