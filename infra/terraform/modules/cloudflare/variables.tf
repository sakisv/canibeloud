variable "zone_name" {
  type = string
}

variable "ssl_mode" {
  type        = string
  description = "SSL mode for talking to the origin"

  validation {
    condition     = contains(["off", "flexible", "full", "strict"], var.ssl_mode)
    error_message = "Valid values for ssl_mode are: ('off', 'flexible', 'full', 'strict')."
  }
}
