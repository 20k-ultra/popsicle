variable "iam_role" {
  type = object({
    name = string
    arn  = string
  })
}

variable "zip_path" {
  description = "Path to ZIP containing Lambda source"
  type    = string
  default = "bootstrap.zip"
}
