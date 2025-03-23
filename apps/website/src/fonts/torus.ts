import localFont from "next/font/local";

export const Torus = localFont({
    src: [
        {
            path: "../../public/fonts/torus/Torus-Regular.otf",
            weight: "400"
        },
        {
            path: "../../public/fonts/torus/Torus-SemiBold.otf",
            weight: "500"
        },
        {
            path: "../../public/fonts/torus/Torus-Bold.otf",
            weight: "600"
        },
        {
            path: "../../public/fonts/torus/Torus-Heavy.otf",
            weight: "700"
        },
        {
            path: "../../public/fonts/torus/Torus-Light.otf",
            weight: "300"
        },
        {
            path: "../../public/fonts/torus/Torus-Thin.otf",
            weight: "200"
        }
    ]
});
