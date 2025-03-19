"use client";

import { Popover } from "radix-ui";
import { motion, AnimatePresence } from "motion/react";
import { useState } from "react";

export default function PopoverDemo() {
    const [open, setOpen] = useState(false);

    return (
        <Popover.Root onOpenChange={setOpen}>
            <Popover.Trigger className="rounded-md border-surface-solid-2 outline-1 hover:surface-1 transition-colors cursor-pointer">
                More info
            </Popover.Trigger>
            <AnimatePresence>
                {open && (
                    <Popover.Portal forceMount>
                        <Popover.Content
                            asChild
                            className="rounded-md m-1 p-5 w-64 surface-solid-1 shadow-sm"
                            collisionPadding={{ bottom: 5, top: 5, left: 5, right: 5 }}
                        >
                            <motion.div
                                animate="show"
                                exit="hidden"
                                initial="hidden"
                                variants={{
                                    hidden: { opacity: 0, y: -5 },
                                    show: { opacity: 1, y: 0 }
                                }}
                            >
                                Some more info...
                            </motion.div>
                        </Popover.Content>
                    </Popover.Portal>
                )}
            </AnimatePresence>
        </Popover.Root>
    );
}
