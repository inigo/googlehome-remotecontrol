Use a BroadLink RM 3 Mini IR Blaster to remote control my TV and speakers via a Google Home.

## Setup

Most of the hard work is done by the BlackBeanControl Python
library, by Davorf - see https://github.com/davorf/BlackBeanControl;
and by the Google Home server-side code.

Install BlackBeanControl. following the instructions in that repo, and
adjust the code to point to it. Train it with appropriate remote control
actions.

To fulfill Google Home requests, you need to be using HTTPS, and you can't
use a self-signed certificate. I'm using a domain name purchased via and hosted
by AWS Route 53, and a certificate from Let's Encrypt, installed in Nginx
that's set up to proxy this Rust app.

I had a little pain stopping OS X's firewall from blocking Nginx, that I got round via:
{{{
    sudo /usr/libexec/ApplicationFirewall/socketfilterfw --add /usr/local/Cellar/nginx/1.12.2_1/bin/nginx
    sudo /usr/libexec/ApplicationFirewall/socketfilterfw --unblockapp /usr/local/Cellar/nginx/1.12.2_1/bin/nginx
    sudo brew services start nginx
}}}

I also needed to configure my router to allow external incoming requests to Nginx.

On the Google Home developer site (https://console.actions.google.com/u/0/), I set up a project with
actions from DialogFlow, with various intents like "Turn on TV" that map to action names like
"TvPowerOn", setting the fulfillment URL to this Rust server, via the Nginx proxy.

## Licence

This code is Copyright (C) 2018 Inigo Surguy.

It is Free Software usable under the terms of the GNU General Public Licence,
version 3 or later - see LICENCE.txt.