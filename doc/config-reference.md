## Configuration reference

Bifrost

```yaml
# Bifrost section [optional!]
#
# Contains bifrost server settings
# [usually omitted, to use defaults]
bifrost:
  # name of yaml file to write state database to
  state_file: "state.yaml"

  # name of x509 certificate for https
  #
  # if this file is missing, bifrost will generate one for you
  #
  # if this file exists, bifrost will check that the mac address
  # matches the specified server mac address
  #
  # to generate a fresh certificate, rename/move this file
  # (this might require pairing the Hue App again)
  cert_file: "cert.pem"

# Bridge section
#
# Settings for hue bridge emulation
bridge:
  name: Bifrost
  mac: 00:11:22:33:44:55
  ipaddress: 10.0.0.12
  netmask: 255.255.255.0
  gateway: 10.0.0.1
  timezone: Europe/Copenhagen

  # HTTP port for emulated bridge
  #
  # beware: most client programs do NOT support non-standard ports.
  # This is for advanced users (e.g. bifrost behind a reverse proxy)
  http_port: 80

  # HTTPS port for emulated bridge
  #
  # beware: most client programs do NOT support non-standard ports.
  # This is for advanced users (e.g. bifrost behind a reverse proxy)
  https_port: 443

  # DTLS port for emulated bridge (Hue Entertainment streaming)
  #
  # beware: client programs do NOT support non-standard ports.
  # For advanced users (e.g. bifrost behind a port forwarded firewall)
  entm_port: 2100

# Zigbee2mqtt section
#
# Make a sub-section for each zigbee2mqtt server you want to connect
#
# The server names ("some-server", "other-server") are used for logging,
# but have no functional impact. Just be sure to pick different names,
# if you have more than one server.
z2m:
  some-server:
    # The websocket url for z2m, starting with "ws://".
    #
    # For z2m version 2.x, the url must end in `/api?token=<token>`.
    # For z2m version 1.x, this is optional, but supported.
    #
    # Therefore, Bifrost will adjust the urls if needed.
    # A message will be logged with the rewritten url if this happens.
    url: ws://10.00.0.100:8080
  other-server:
    url: ws://10.10.0.102:8080

    # Group prefix [optional!]
    #
    # If you specify this parameter, *only* groups with this prefix
    # will be visible from this z2m server. The prefix will be removed.
    #
    # So with a group_prefix of "bifrost_", the group "bifrost_kitchen"
    # will be available as "kitchen", but the group "living_room" will
    # be hidden instead.
    group_prefix: bifrost_
  ...

# Rooms section [optional!]
#
# This section allows you to map zigbee2mqtt "friendly names" to
# a human-readable description you provide.
#
# Each entry under "rooms" must match a zigbee2mqtt "friendly name",
# and can contain the following keys: (both are optional)
#
#   name: The human-readable name presented in the API (for the Hue App, etc)
#
#   icon: The icon to use for this room. Must be selected from the following
#         list of icons supported by the Hue App:
#
#         attic balcony barbecue bathroom bedroom carport closet computer dining
#         downstairs driveway front_door garage garden guest_room gym hallway
#         home kids_bedroom kitchen laundry_room living_room lounge man_cave
#         music nursery office other pool porch reading recreation staircase
#         storage studio terrace toilet top_floor tv upstairs
#
rooms:
  office_group:
    name: Office 1
    icon: office

  carport_group:
    name: Carport Lights
    icon: carport

  ...
```
