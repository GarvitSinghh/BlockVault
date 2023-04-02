import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Blockvault } from "../target/types/blockvault";

describe("blockvault", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Blockvault as Program<Blockvault>;

    it("can upload file!", async () => {
        // Add your test here.
        const file = anchor.web3.Keypair.generate();
        await program.methods
            .uploadFile("Test Title", "Test Description", "Test CID")
            .accounts({
                file: file.publicKey,
                owner: program.provider.publicKey,
            })
            .signers([file])
            .rpc();
    });

    it("can upload file from different user", async () => {
        const file = anchor.web3.Keypair.generate();
        const otherUser = anchor.web3.Keypair.generate();
        const signature = await program.provider.connection.requestAirdrop(
            otherUser.publicKey,
            1000000000
        );
        await program.provider.connection.confirmTransaction(signature);

        await program.methods
            .uploadFile("Test Title Other User", "Test Description from the Other User", "Test CID from Other User")
            .accounts({
                file: file.publicKey,
                owner: otherUser.publicKey,
            })
            .signers([otherUser, file])
            .rpc();
    })

    it("can fetch all files", async () => {
        const files = await program.account.file.all()
        console.log(files);
    })
});
