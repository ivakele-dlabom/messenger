import {useState} from "react";
import * as React from "react";

interface Entry {
    x: number;
    y: number;
}

export const Entry = () => {
    const [entry, setEntry] = useState<Entry>({
        x: 0,
        y: 0,
    });

    const handleSubmit = async (_:  React.SubmitEvent<HTMLFormElement>) => {
        const response = await fetch("http://localhost:8081/entries", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(entry),
        });

        if (!response.ok) {
            console.error("Failed to save entry");
            return;
        }

        const data = await response.json(); // if your backend returns JSON
        console.log(data);


    }

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;

        setEntry((prev) => ({
            ...prev,
            [name]: Number(value),
        }));
    };
    return (
        <div>
            <form onSubmit={handleSubmit} className="flex flex-col gap-3 p-4 max-w-sm">
                <div className="flex flex-col gap-1">
                    <label htmlFor="x" className="text-sm font-medium text-gray-700">
                        X
                    </label>
                    <input
                        id="x"
                        name="x"
                        onChange={handleInputChange}
                        className="h-10 w-full rounded-md border border-gray-300 px-3 text-sm outline-none transition focus:border-cyan-500 focus:ring-2 focus:ring-cyan-100"
                    />
                </div>

                <div className="flex flex-col gap-1">
                    <label htmlFor="y" className="text-sm font-medium text-gray-700">
                        Y
                    </label>
                    <input
                        id="y"
                        name="y"
                        onChange={handleInputChange}
                        className="h-10 w-full rounded-md border border-gray-300 px-3 text-sm outline-none transition focus:border-cyan-500 focus:ring-2 focus:ring-cyan-100"
                    />
                </div>

                <button
                    type="submit"
                    className="mt-1 h-10 rounded-md bg-cyan-600 text-sm font-medium text-white transition hover:bg-cyan-700 active:bg-cyan-800"
                >
                    Submit
                </button>
            </form>
        </div>
    )
}

