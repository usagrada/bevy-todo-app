import { atom } from 'recoil';

interface Channels {
    channels: Channel[];
    select: string;
}

interface Channel {
    name: string;
}

export const atom_channels = atom<Channels>({
    key: 'channels',
    default: { channels: [
        { name: 'general' },
        { name: 'random' },
    ], select: '' },
});
