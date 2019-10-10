killall examsafety
nohup ./target/release/examsafety > examsafety.log &
tail -f examsafety.log
