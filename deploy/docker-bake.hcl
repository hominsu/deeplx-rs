target "metadata" {}

group "default" {
  targets = [
    "deeplx",
  ]
}

target "cross" {
  platforms = [
    "linux/arm64",
    "linux/amd64"
  ]
}

target "deeplx" {
  inherits = ["metadata", "cross"]
  cache-from = ["type=gha"]
  cache-to = ["type=gha,mode=max"]
  context    = "."
  dockerfile = "deploy/Dockerfile"
}
