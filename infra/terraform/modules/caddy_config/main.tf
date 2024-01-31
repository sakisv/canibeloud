variable "domain" { type = string }
variable "cloudflare_caddy_api_token" { type = string }
variable "proxy_addr_and_port" { type = string }

output "caddyfile_b64" {
  value = base64encode(
    templatefile("${path.module}/templates/caddy.tmpl", {
      domain                     = var.domain
      proxy_addr_and_port        = var.proxy_addr_and_port
      cloudflare_caddy_api_token = var.cloudflare_caddy_api_token
    })
  )
}
