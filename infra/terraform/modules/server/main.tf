locals {
  image = "ubuntu-22.04"
}

resource "random_string" "server_name_suffix" {
  length  = 6
  special = false
  numeric = true
  upper   = false
}

resource "hcloud_server" "web" {
  name        = "${var.name_prefix}-${random_string.server_name_suffix.result}"
  image       = local.image
  server_type = var.server_type
  location    = var.location
  ssh_keys    = var.ssh_key_ids
  user_data = templatefile("${path.module}/userdata/userdata.yml", {
    public_bucket_url     = var.public_bucket_url
    custom_caddy_filename = var.custom_caddy_filename
  })
  firewall_ids = [hcloud_firewall.web_and_ssh.id]
}

resource "cloudflare_record" "web" {
  for_each = {
    for i, record in var.dns_records :
    # this returns a dict in the form of
    # "@" = { "name": "@", "use_proxy": false}
    record.name => record
  }
  zone_id = var.cloudflare_zone_id
  name    = each.value.name
  type    = "A"
  value   = hcloud_server.web.ipv4_address
  ttl     = each.value.use_proxy ? 1 : var.cloudflare_ttl
  proxied = each.value.use_proxy
}
