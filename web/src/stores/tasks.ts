import { channel } from 'diagnostics_channel';
import { atom } from 'recoil';

interface Channels {
    channels: Channel[];
    select: string;
}

interface Channel {
    name: string;
    lists: string[];
}

export const atom_channels = atom<Channels>({
    key: 'channels',
    default: {
        channels: [
            { name: 'general', lists: ['list1', 'list2', 'list3', 'list4', 'list5'] },
            { name: 'random', lists: ['TODO', 'DONE'] },
        ],
        select: '',
    },
});
