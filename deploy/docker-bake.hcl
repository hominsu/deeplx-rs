target "metadata" {}

group "default" {
  targets = [
    "dlx-svr",
  ]
}

target "cross" {
  platforms = [
    "linux/arm64",
    "linux/amd64"
  ]
}

target "dlx-svr" {
  inherits = ["metadata", "cross"]
  context    = "."
  dockerfile = "deploy/Dockerfile"
}
