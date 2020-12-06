# this script use to build proto
# (windows only)

$files = Get-ChildItem src/proto
foreach ($file in $files) {
  # build proto
./tools/protoc.exe `
  --proto_path=src/proto `
  --rust_out=src/proto_rs `
  $file
}