"use client";

import { useEffect, useRef } from "react";
import Paper from "paper";

export default function Canvas() {
    const canvasRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        const canvas = canvasRef.current;
        Paper.setup(canvas!);

        const myPath = new Paper.Path();

        Paper.view.onMouseDown = () => {
            myPath.strokeColor = new Paper.Color("white");
            myPath.strokeWidth = 3;
        };

        Paper.view.onMouseDrag = (e: paper.MouseEvent) => {
            myPath.add(e.point);
        };
    }, []);

    return <canvas ref={canvasRef} width="100%" height="100%" />;
}
