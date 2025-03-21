"use client";

import { useEffect, useRef } from "react";
import Paper from "paper";
import { PaperRoundCorners } from "paperjs-round-corners";

interface SectionProps {
    preText?: string;
    iconGlow?: boolean;
    icon: React.ReactElement;
    children: React.ReactNode;
}

export function Section(props: SectionProps) {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const preTextRef = useRef<HTMLParagraphElement>(null);
    const iconRef = useRef<HTMLDivElement>(null);
    const contentRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const canvas = canvasRef.current;
        Paper.setup(canvas!);

        const path = new Paper.Path();
        const preText = preTextRef.current;
        const firstLineXWidth = preText?.getBoundingClientRect().width ?? 0;
        const topLineWidth = firstLineXWidth + 10 + (preText !== null ? 15 : 0);

        path.strokeColor = new Paper.Color(1, 1, 1, 0.5);
        path.strokeWidth = 2;
        path.add(
            new Paper.Point(0, 20),
            new Paper.Point(topLineWidth, 20),
        )

        const canvasHeight = canvas?.getBoundingClientRect().height ?? 0;
        const lineHeight = canvasHeight - (20 * 2);

        path.lineBy(new Paper.Point(lineHeight, lineHeight));
        PaperRoundCorners.round(path.segments[1], 5);

        const content = contentRef.current;
        if (content === null) {
            return;
        }

        const contentWidth = content.getBoundingClientRect().width;

        path.lineBy(new Paper.Point(contentWidth + 20, 0));
        PaperRoundCorners.round(path.segments[3], 5);
    }, []);

    return (
        <div className="relative flex flex-row gap-1 items-center h-full">
            <canvas ref={canvasRef} className="absolute -top-2 -left-2 h-20 w-[calc(100%+--spacing(2))] pointer-events-none" />

            {props.preText && <p ref={preTextRef} className="text-xl mr-4 tracking-[15%] glow max-md:hidden">{props.preText}</p>}
            <div className="flex flex-row items-end gap-4 pl-2">
                <div ref={iconRef} className={`h-max w-max ${props.iconGlow && "glow"}`}>
                    {props.icon}
                </div>
                <div ref={contentRef} className="flex flex-row gap-2">
                    {props.children}
                </div>
            </div>
        </div>
    )
}
