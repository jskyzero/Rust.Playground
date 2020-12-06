# this script use to build proto
# (windows only)

$files = Get-ChildItem config/proto
foreach ($file in $files) {
  # build proto
./tools/protoc.exe `
  --proto_path=config/proto `
  --rust_out=config/src `
  $file
}