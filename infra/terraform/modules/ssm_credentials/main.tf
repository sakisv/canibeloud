resource "aws_ssm_parameter" "param" {
  for_each = var.secrets

  name  = "${var.common_prefix}${each.key}"
  value = each.value
  type  = "SecureString"
}

variable "secrets" {
  type = map(string)
}

variable "common_prefix" {
  type = string
}
