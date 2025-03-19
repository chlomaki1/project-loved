"use client";

import { Popover } from "radix-ui";
import { motion, AnimatePresence } from "motion/react";
import { useState } from "react";

export default function PopoverDemo() {
    const [open, setOpen] = useState(false);

    return (
        <Popover.Root onOpenChange={setOpen}>
            <Popover.Trigger className="rounded-md bg-white" onClick={() => setOpen(prev => !prev)}>
                More info
            </Popover.Trigger>
            <AnimatePresence>
                {open && (
                    <Popover.Portal forceMount>
                        <Popover.Content asChild className="rounded-md p-5 w-64 bg-red-100">
                            <motion.div
                                animate="show"
                                exit="hidden"
                                initial="hidden"
                                variants={{
                                    hidden: { opacity: 0, y: 10 },
                                    show: { opacity: 1, y: 0 }
                                }}
                            >
                                Some more info...
                                <Popover.Arrow className="fill-red-100" />
                            </motion.div>
                        </Popover.Content>
                    </Popover.Portal>
                )}
            </AnimatePresence>
        </Popover.Root>
    );
}
