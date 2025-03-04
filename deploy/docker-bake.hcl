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
  context    = "."
  dockerfile = "deploy/Dockerfile"
}
