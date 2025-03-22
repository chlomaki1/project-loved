"use client";

import { createContext, useEffect, useMemo, useRef } from "react";
import Paper from "paper";
import { PaperRoundCorners } from "paperjs-round-corners";

interface NavbarSectionContext {
    currentElement: HTMLElement | null;
}

const NavbarSectionContext = createContext<NavbarSectionContext | null>(null);

interface SectionProps {
    preText?: string;
    iconGlow?: boolean;
    debug?: boolean;
    iconTopSize: number;
    iconBottomSize: number;
    icon: React.ReactElement;
    children: React.ReactNode;
}

export function Section(props: SectionProps) {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const preTextRef = useRef<HTMLParagraphElement>(null);
    const iconRef = useRef<HTMLDivElement>(null);
    const contentRef = useRef<HTMLDivElement>(null);

    const sectionContext = useMemo<NavbarSectionContext | null>(
        () => ({ currentElement: null } satisfies NavbarSectionContext),
        []
    )

    useEffect(() => {
        window.addEventListener("resize", draw);
        draw();
        return () => window.removeEventListener("resize", draw);
    }, [sectionContext?.currentElement]);

    function draw() {
        const canvas = canvasRef.current;
        Paper.setup(canvas!);

        const path = new Paper.Path()
        const group = new Paper.CompoundPath({
            children: [path],
            selected: props.debug ?? false,
            strokeColor: new Paper.Color(1, 1, 1, 0.5),
            strokeWidth: 2
        });
        const preText = preTextRef.current;
        const firstLineXWidth = preText?.getBoundingClientRect().width ?? 0;
        const topLineWidth = firstLineXWidth + 10 + (preText !== null && firstLineXWidth !== 0 ? 15 : preText !== null ? -4 : 0);

        path.add(
            new Paper.Point(0, 20),
            new Paper.Point(topLineWidth, 20),
        )

        const canvasHeight = canvas?.getBoundingClientRect().height ?? 0;
        const lineHeight = (canvasHeight - (20 * 2)) / 2;

        path.lineBy(new Paper.Point(lineHeight - props.iconTopSize, lineHeight - props.iconTopSize));
        PaperRoundCorners.round(path.segments[1], 5);

        group.moveBy(new Paper.Point(props.iconTopSize + props.iconBottomSize, props.iconTopSize + props.iconBottomSize));

        const path2 = new Paper.Path()
        group.addChild(path2);

        path2.moveTo(group.bounds.bottomRight);
        path2.lineBy(new Paper.Point(0, 0));
        path2.lineTo(new Paper.Point(topLineWidth + (lineHeight * 2), 20 + (lineHeight * 2)));

        const content = contentRef.current;
        if (content === null) {
            return;
        }

        if (sectionContext?.currentElement === null) {
            const contentWidth = content.getBoundingClientRect().width;
            path2.lineBy(new Paper.Point(contentWidth + 20, 0));
        } else if (sectionContext?.currentElement !== null) {
            const contentBounds = content.getBoundingClientRect();
            const currentElementBounds = sectionContext!.currentElement.getBoundingClientRect();

            const margin = 12;

            const leftWidth = currentElementBounds.left - contentBounds.left;
            const rightWidth = contentBounds.right - currentElementBounds.right;
            const gap = contentBounds.width - leftWidth - rightWidth;

            path2.lineBy(new Paper.Point(18 - (margin / 2), 0));
            path2.lineBy(new Paper.Point(leftWidth, 0));

            const path3 = new Paper.Path()
            group.addChild(path3);

            path3.moveTo(new Paper.Point(path2.bounds.right + gap + margin + 1, path2.bounds.bottom));
            path3.lineBy(new Paper.Point(0, 0));
            path3.lineTo(new Paper.Point(path3.bounds.right + 20 + rightWidth, path3.bounds.bottom));
        }

        PaperRoundCorners.round(path2.segments[2], 5);
    }

    return (
        <div className="relative flex flex-row gap-1 items-center h-full">
            <canvas ref={canvasRef} className="absolute -top-2 -left-2 h-20 w-[calc(100%+--spacing(6))] pointer-events-none" />

            {props.preText && <p ref={preTextRef} className="text-xl mr-4 tracking-[15%] glow max-lg:hidden">{props.preText}</p>}
            <div className="flex flex-row items-end gap-6 pl-2">
                <div ref={iconRef} className={`h-max w-max ${props.iconGlow ? "glow" : "opacity-50"}`}>
                    {props.icon}
                </div>
                <div ref={contentRef} className="flex flex-row gap-4">
                    <NavbarSectionContext.Provider value={sectionContext}>
                        {props.children}
                    </NavbarSectionContext.Provider>
                </div>
            </div>
        </div>
    )
}

Section.Context = NavbarSectionContext;
