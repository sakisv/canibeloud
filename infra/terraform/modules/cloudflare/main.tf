data "cloudflare_zone" "zone" {
  name = var.zone_name
}

resource "cloudflare_zone_settings_override" "ssl" {
  zone_id = data.cloudflare_zone.zone.id

  settings {
    ssl = var.ssl_mode
  }
}
