variable "name_prefix" {
  type = string
}

variable "server_type" {
  type = string
}

variable "location" {
  type = string
}

variable "dns_records" {
  type = list(
    object({
      name      = string
      use_proxy = bool
      }
    )
  )
}

variable "ssh_key_ids" {
  type = list(string)
}

variable "cloudflare_zone_id" {
  type = string
}

variable "cloudflare_ttl" {
  default     = 300
  type        = number
  description = "TTL for cloudflare records, defaults to 300s (5m)"
}

variable "environment" {
  type = string
}
