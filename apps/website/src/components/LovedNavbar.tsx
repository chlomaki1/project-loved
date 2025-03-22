"use client";

import Icon from "../../public/icon.svg";
import Image from "next/image";
import { Navbar } from "@loved/ui";
import { UserLove } from "iconoir-react";

export default function LovedNavbar() {
    return (
        <Navbar>
            <Navbar.Section iconGlow preText="PROJECT LOVED" icon={<Image src={Icon} width={24} height={24} alt="project loved" />} iconTopSize={7 + 3} iconBottomSize={8 + 3}>
                <Navbar.Item href="/">Submissions</Navbar.Item>
                <Navbar.Item href="/consents">Mapper consents</Navbar.Item>
                <Navbar.Item href="/team">Team</Navbar.Item>
                <Navbar.Item href="/statistics">Statistics</Navbar.Item>
            </Navbar.Section>
            <Navbar.Section icon={<UserLove width={24} height={24} />} iconTopSize={7 + 3} iconBottomSize={8 + 3}>
                <Navbar.Item href="/admin">Admin panel</Navbar.Item>
            </Navbar.Section>
        </Navbar>
    );
}
