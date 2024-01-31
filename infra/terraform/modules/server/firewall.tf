resource "random_pet" "firewall_name" {
  length = 1
  prefix = var.environment
}

resource "hcloud_firewall" "web_and_ssh" {
  name = random_pet.firewall_name.id
  rule {
    description = "allow ssh"
    direction   = "in"
    protocol    = "tcp"
    port        = "22"
    source_ips = [
      "0.0.0.0/0",
      "::/0"
    ]
  }

  rule {
    description = "allow TLS traffic"
    direction   = "in"
    protocol    = "tcp"
    port        = "443"
    source_ips = [
      "0.0.0.0/0",
      "::/0"
    ]
  }

  rule {
    description = "allow unencrypted traffic (for cert generation)"
    direction   = "in"
    protocol    = "tcp"
    port        = "80"
    source_ips = [
      "0.0.0.0/0",
      "::/0"
    ]
  }

  labels = {
    Name        = "web-and-ssh"
    Environment = var.environment
  }
}
