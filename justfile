
bump-major:
  cargo next --major

bump-minor:
  cargo next --minor

bump-patch:
  cargo next --patch

generate-major-tag: prep-major-release
  git tag $(cargo get package.version --pretty)

generate-minor-tag: prep-minor-release
  git tag $(cargo get package.version --pretty)

generate-patch-tag: prep-patch-release
  git tag $(cargo get package.version --pretty)

find-msrv:
  cargo msrv find --linear

prep-major-release: bump-major
  git add .
  git commit -m "Released $(cargo get package.version --pretty)"

prep-minor-release: bump-minor
  git add .
  git commit -m "Released $(cargo get package.version --pretty)"

prep-patch-release: bump-patch
  git add .
  git commit -m "Released $(cargo get package.version --pretty)"

