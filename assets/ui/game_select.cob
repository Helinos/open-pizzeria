#defs
$inconsolata = {
    family: "Inconsolata"
}

$inconsolata_bold = {
    family: "Inconsolata"
    weight: Bold
}

$button_idle = #666666
$button_hover = #888888
$button_press = #444444
$button_padding = { top: 2px bottom: 3px left: 10px right: 10px }
$button_spacing = { top: 20px }

#commands
RegisterFontFamilies[
    {
        family: "Inconsolata"
        fonts:[
            {
                path: "fonts/Inconsolata-Regular.ttf"
                width: Normal
                style: Normal
                weight: Normal
            }
            {
                path: "fonts/Inconsolata-Bold.ttf"
                width: Normal
                style: Normal
                weight: Bold
            }
        ]
    }
]

LoadFonts[
    "Inconsolata"
]

#scenes
"root"
    FlexNode{ width: 100% height: 100% justify_cross: Center flex_direction: Column}
        
    "title"
        FlexNode{ margin: { top: 15% } }
        TextLine{ text: "Select a Game" font: $inconsolata size: 64}

    "buttons"
        FlexNode{ margin: {top: 5%} justify_cross: Center flex_direction: Column }

        "fnaf1"
            FlexNode{ margin: $button_spacing }
            Animated<BackgroundColor>{ idle: $button_idle hover: $button_hover press: $button_press }
            
            "text"
                FlexNode{ margin: $button_padding }
                TextLine{ text: "Five Nights at Freddy's" font: $inconsolata_bold}

        // "fnaf2"
        //     FlexNode{ margin: $button_spacing }
        //     Animated<BackgroundColor>{ idle: $button_idle hover: $button_hover press: $button_press }
            
        //     "text"
        //         FlexNode{ margin: $button_padding }
        //         TextLine{ text: "Five Nights at Freddy's 2" font: $inconsolata_bold}
        
        // "fnaf3"
        //     FlexNode{ margin: $button_spacing }
        //     Animated<BackgroundColor>{ idle: $button_idle hover: $button_hover press: $button_press }
            
        //     "text"
        //         FlexNode{ margin: $button_padding }
        //         TextLine{ text: "Five Nights at Freddy's 3" font: $inconsolata_bold}
    
