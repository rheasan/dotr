set -xe
rm -rf ~/.dotr
rm -rf ~/.config/dotr_tests
mkdir ~/.config/dotr_tests

cargo run --quiet -- init
cargo run --quiet -- add README.md ~/.config/dotr_tests -n readme -d "a readme file" --symlink
cargo run --quiet -- add .gitignore ~/.config/dotr_tests -n ignore -d gitignore

echo "----------------------------------------------"
ls -la ~/.config/dotr_tests

echo "----------------------------------------------"
ls -la ~/.dotr

echo "----------------------------------------------"
cat ~/.dotr/dotr.json | jq

echo "----------------------------------------------"
echo "test proper data update when a file is updated"
yes | cargo run --quiet -- add README.md ~/.config/dotr_tests -n readme -d "an updated readme file" --symlink
cat ~/.dotr/dotr.json | jq