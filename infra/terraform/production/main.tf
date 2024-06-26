data "pass_password" "public_bucket_url" { path = "credentials/cloudflare/public_bucket_url" }
data "pass_password" "cloudflare_caddy_api_token" { path = "credentials/cloudflare/caddy_api_token" }

module "server" {
  source      = "../modules/server"
  name_prefix = "canibeloud-production"
  server_type = "cax11" # arm64 - 2vCPU, 4GB RAM, 40GB ssd
  location    = "hel1"
  dns_records = [
    { "name" : "@", "use_proxy" : true },
    { "name" : "www", "use_proxy" : false },
  ]
  public_bucket_url  = data.pass_password.public_bucket_url.password
  cloudflare_zone_id = module.canibeloud_zone.zone_id
  ssh_key_ids        = module.ssh_keys.key_ids
  environment        = "production"
  b64_caddyfile      = module.caddyfile.caddyfile_b64
}

module "canibeloud_zone" {
  source    = "../modules/cloudflare"
  zone_name = "canibeloud.com"
  ssl_mode  = "strict"
}

module "caddyfile" {
  source                     = "../modules/caddy_config"
  domain                     = "canibeloud.com"
  cloudflare_caddy_api_token = data.pass_password.cloudflare_caddy_api_token.password
  proxy_addr_and_port        = "127.0.0.1:8080"
}

module "ssh_keys" {
  source = "../modules/hetzner_ssh_keys"
}
