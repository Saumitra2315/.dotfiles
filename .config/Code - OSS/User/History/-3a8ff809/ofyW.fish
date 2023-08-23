#!/bin/fish

set EWW (which eww)

# Run eww daemon if not running already
if not pidof eww
    $EWW daemon
    sleep 1
end

# Open widgets
function run_eww
    $EWW -c $HOME/.config/eww/vertical-bar close bar
    $EWW -c $HOME/.config/eww/vertical-bar open bar
end

# Launch or close widgets accordingly
run_eww
