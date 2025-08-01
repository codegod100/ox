/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.rs", "./dist/**/*.html"],
  safelist: [
    // Ensure all Catppuccin colors are included
    {
      pattern: /^(bg|text|border)-ctp-.*/,
    },
    // Include gradient patterns
    {
      pattern: /^(bg-gradient|from|via|to)-ctp-.*/,
    },
    // Include animations
    "animate-float",
    "animate-shimmer",
    "animate-glow-pulse",
    "animate-slide-in",
    "animate-fade-in",
    "animate-scale-in",
    "animate-shine",
    // Include backdrop blur
    "backdrop-blur-xl",
    "backdrop-blur-2xl",
    "backdrop-blur-4xl",
    // Include shadows
    "shadow-glow",
    "shadow-glow-lg",
    "shadow-glass",
    "shadow-inner-glow",
  ],
  theme: {
    extend: {
      colors: {
        "ctp-rosewater": "#f5e0dc",
        "ctp-flamingo": "#f2cdcd",
        "ctp-pink": "#f5c2e7",
        "ctp-mauve": "#cba6f7",
        "ctp-red": "#f38ba8",
        "ctp-maroon": "#eba0ac",
        "ctp-peach": "#fab387",
        "ctp-yellow": "#f9e2af",
        "ctp-green": "#a6e3a1",
        "ctp-teal": "#94e2d5",
        "ctp-sky": "#89dceb",
        "ctp-sapphire": "#74c7ec",
        "ctp-blue": "#89b4fa",
        "ctp-lavender": "#b4befe",
        "ctp-text": "#cdd6f4",
        "ctp-subtext1": "#bac2de",
        "ctp-subtext0": "#a6adc8",
        "ctp-overlay2": "#9399b2",
        "ctp-overlay1": "#7f849c",
        "ctp-overlay0": "#6c7086",
        "ctp-surface2": "#585b70",
        "ctp-surface1": "#45475a",
        "ctp-surface0": "#313244",
        "ctp-base": "#1e1e2e",
        "ctp-mantle": "#181825",
        "ctp-crust": "#11111b",
      },
      animation: {
        float: "float 6s ease-in-out infinite",
        shimmer: "shimmer 3s ease-in-out infinite",
        "glow-pulse": "glow-pulse 2s ease-in-out infinite",
        "slide-in": "slide-in 0.5s ease-out",
        "fade-in": "fade-in 0.3s ease-out",
        "scale-in": "scale-in 0.2s ease-out",
        shine: "shine 1s ease-in-out",
      },
      keyframes: {
        float: {
          "0%, 100%": { transform: "translateY(0px)" },
          "50%": { transform: "translateY(-10px)" },
        },
        shimmer: {
          "0%": { backgroundPosition: "-200% 0" },
          "100%": { backgroundPosition: "200% 0" },
        },
        "glow-pulse": {
          "0%, 100%": { boxShadow: "0 0 20px rgba(203, 166, 247, 0.3)" },
          "50%": { boxShadow: "0 0 40px rgba(203, 166, 247, 0.6)" },
        },
        "slide-in": {
          "0%": { transform: "translateY(20px)", opacity: "0" },
          "100%": { transform: "translateY(0)", opacity: "1" },
        },
        "fade-in": {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
        "scale-in": {
          "0%": { transform: "scale(0.95)", opacity: "0" },
          "100%": { transform: "scale(1)", opacity: "1" },
        },
        shine: {
          "0%": { transform: "translateX(-100%) skewX(-45deg)" },
          "100%": { transform: "translateX(200%) skewX(-45deg)" },
        },
      },
      backgroundImage: {
        "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
        "gradient-conic":
          "conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))",
        "shimmer-gradient":
          "linear-gradient(90deg, transparent, rgba(255,255,255,0.1), transparent)",
      },
      backdropBlur: {
        xs: "2px",
        "4xl": "72px",
      },
      boxShadow: {
        glow: "0 0 20px rgba(203, 166, 247, 0.3)",
        "glow-lg": "0 0 40px rgba(203, 166, 247, 0.4)",
        "inner-glow": "inset 0 2px 4px rgba(255, 255, 255, 0.1)",
        glass:
          "0 8px 32px rgba(0, 0, 0, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.1)",
      },
      fontFamily: {
        mono: ["JetBrains Mono", "Fira Code", "Consolas", "monospace"],
      },
    },
  },
  plugins: [],
};
